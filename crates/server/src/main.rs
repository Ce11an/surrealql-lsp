use std::sync::Arc;
use tokio::sync::Mutex;
use tree_sitter::Point;

struct CompletionDetails<'a> {
    keyword: &'a str,
    documentation: &'a str,
}

fn get_target_options_completion_details() -> Vec<CompletionDetails<'static>> {
    vec![
        CompletionDetails {
            keyword: "WHERE",
            documentation: include_str!("./md/where.md"),
        },
        CompletionDetails {
            keyword: "SPLIT",
            documentation: include_str!("./md/split.md"),
        },
        CompletionDetails {
            keyword: "WITH",
            documentation: include_str!("./md/with.md"),
        },
        CompletionDetails {
            keyword: "GROUP BY",
            documentation: include_str!("./md/group_by.md"),
        },
        CompletionDetails {
            keyword: "LIMIT",
            documentation: include_str!("./md/limit.md"),
        },
        CompletionDetails {
            keyword: "ORDER BY",
            documentation: include_str!("./md/order_by.md"),
        },
        CompletionDetails {
            keyword: "TIMEOUT",
            documentation: include_str!("./md/timeout.md"),
        },
        CompletionDetails {
            keyword: "EXPLAIN",
            documentation: include_str!("./md/explain.md"),
        },
        CompletionDetails {
            keyword: "PARALLEL",
            documentation: include_str!("./md/parallel.md"),
        },
    ]
}

fn get_select_options_completion_details() -> Vec<CompletionDetails<'static>> {
    vec![CompletionDetails {
        keyword: "VALUE",
        documentation: include_str!("./md/value.md"),
    }]
}

fn get_select_completion_details() -> Vec<CompletionDetails<'static>> {
    vec![CompletionDetails {
        keyword: "SELECT",
        documentation: include_str!("./md/select.md"),
    }]
}

fn create_completion_item(
    keyword: &str,
    documentation: &str,
) -> tower_lsp::lsp_types::CompletionItem {
    tower_lsp::lsp_types::CompletionItem {
        label: keyword.to_string(),
        kind: Some(tower_lsp::lsp_types::CompletionItemKind::KEYWORD),
        documentation: Some(tower_lsp::lsp_types::Documentation::MarkupContent(
            tower_lsp::lsp_types::MarkupContent {
                kind: tower_lsp::lsp_types::MarkupKind::Markdown,
                value: documentation.to_string(),
            },
        )),
        ..Default::default()
    }
}

fn get_completion_items(
    completion_details: Vec<CompletionDetails<'static>>,
) -> Vec<tower_lsp::lsp_types::CompletionItem> {
    completion_details
        .iter()
        .map(|detail| create_completion_item(detail.keyword, detail.documentation))
        .collect()
}

fn get_options_to_completion_items_map(
) -> std::collections::HashMap<&'static str, Vec<tower_lsp::lsp_types::CompletionItem>> {
    let mut map = std::collections::HashMap::new();
    map.insert(
        "select",
        get_completion_items(get_select_completion_details()),
    );
    map.insert(
        "select_options",
        get_completion_items(get_select_options_completion_details()),
    );
    map.insert(
        "target_options",
        get_completion_items(get_target_options_completion_details()),
    );
    map
}

fn cursor_matches(
    cursor_line: usize,
    cursor_char: usize,
    query_start: Point,
    query_end: Point,
) -> bool {
    // completely envelop the cursor line-wise
    if query_start.row < cursor_line && query_end.row > cursor_line {
        return true;
    }

    // single line match, check columns on both sides
    if cursor_line == query_start.row
        && cursor_line == query_end.row
        && query_start.column <= cursor_char
        && query_end.column >= cursor_char
    {
        return true;
    }

    // start lines overlap, but the start column is before the cursor column
    if cursor_line == query_start.row && query_start.column <= cursor_char {
        return true;
    }

    // end lines overlap, but the end column is after the cursor column
    if cursor_line == query_end.row && query_end.column >= cursor_char {
        return true;
    }

    false
}

fn cursor_before(cursor_line: usize, cursor_char: usize, query_start: Point) -> bool {
    if cursor_line < query_start.row
        || (cursor_line == query_start.row && cursor_char < query_start.column)
    {
        return true;
    }

    false
}

fn cursor_after(cursor_line: usize, cursor_char: usize, query_end: Point) -> bool {
    if cursor_line > query_end.row
        || (cursor_line == query_end.row && cursor_char > query_end.column)
    {
        return true;
    }

    false
}

pub fn text_doc_change_to_tree_sitter_edit(
    change: &tower_lsp::lsp_types::TextDocumentContentChangeEvent,
    doc: &lsp_textdocument::FullTextDocument,
) -> Result<tree_sitter::InputEdit, &'static str> {
    let range = change.range.ok_or("Invalid edit range")?;
    let start = range.start;
    let end = range.end;

    let start_byte = doc.offset_at(start) as usize;
    let new_end_byte = start_byte + change.text.len();
    let new_end_pos = doc.position_at(new_end_byte as u32);

    Ok(tree_sitter::InputEdit {
        start_byte,
        old_end_byte: doc.offset_at(end) as usize,
        new_end_byte,
        start_position: tree_sitter::Point {
            row: start.line as usize,
            column: start.character as usize,
        },
        old_end_position: tree_sitter::Point {
            row: end.line as usize,
            column: end.character as usize,
        },
        new_end_position: tree_sitter::Point {
            row: new_end_pos.line as usize,
            column: new_end_pos.character as usize,
        },
    })
}

pub struct ServerTextDocumentItem {
    pub uri: tower_lsp::lsp_types::Url,
    pub text: String,
}

fn initialise_parser() -> tree_sitter::Parser {
    let mut parser = tree_sitter::Parser::new();
    if parser
        .set_language(tree_sitter_surrealql::language())
        .is_err()
    {
        panic!("Failed to set parser language");
    }
    parser
}

fn get_completion_list(
    curr_doc: &str,
    parser: &mut tree_sitter::Parser,
    curr_tree: &mut Option<tree_sitter::Tree>,
    params: &tower_lsp::lsp_types::CompletionParams,
    options_to_completions_map: &std::collections::HashMap<
        &'static str,
        Vec<tower_lsp::lsp_types::CompletionItem>,
    >,
) -> Option<Vec<tower_lsp::lsp_types::CompletionItem>> {
    let cursor_line = params.text_document_position.position.line as usize;
    let cursor_char = params.text_document_position.position.character as usize;

    *curr_tree = parser.parse(curr_doc, curr_tree.as_ref());
    if let Some(tree) = curr_tree {
        let mut cursor = tree_sitter::QueryCursor::new();
        let curr_doc = curr_doc.as_bytes();

        static QUERY_INSTR_ANY: once_cell::sync::Lazy<tree_sitter::Query> =
            once_cell::sync::Lazy::new(|| {
                tree_sitter::Query::new(
                    tree_sitter_surrealql::language(),
                    r#"
                    (from_clause (target) @target_options . )

                    (keyword_select) @select_options
                    "#,
                )
                .expect("Could not initialise query")
            });

        let matches_iter = cursor.matches(&QUERY_INSTR_ANY, tree.root_node(), curr_doc);
        let mut last_match: Option<(String, tree_sitter::Range)> = None;

        for match_ in matches_iter {
            for capture in match_.captures.iter() {
                let capture_name = &QUERY_INSTR_ANY.capture_names()[capture.index as usize];
                let arg_start = capture.node.range().start_point;
                let arg_end = capture.node.range().end_point;

                if cursor_matches(cursor_line, cursor_char, arg_start, arg_end) {
                    last_match = Some((capture_name.clone(), capture.node.range()));
                }
            }
        }

        if let Some((capture_name, _range)) = last_match {
            if let Some(completion_items) = options_to_completions_map.get(capture_name.as_str()) {
                return Some(completion_items.clone());
            }
        }

        static QUERY_STATEMEMENT_START: once_cell::sync::Lazy<tree_sitter::Query> =
            once_cell::sync::Lazy::new(|| {
                tree_sitter::Query::new(tree_sitter_surrealql::language(), "(ERROR) @start")
                    .expect("Could not initialise query")
            });

        for match_ in cursor.matches(&QUERY_STATEMEMENT_START, tree.root_node(), curr_doc) {
            for capture in match_.captures.iter() {
                let arg_start = capture.node.range().start_point;
                let arg_end = capture.node.range().end_point;

                if cursor_matches(cursor_line, cursor_char, arg_start, arg_end) {
                    return options_to_completions_map.get("select").cloned();
                }
            }
        }

        // match SELECT and ensure the next neighbor is beyond the cursor
        static QUERY_SELECT_1: once_cell::sync::Lazy<tree_sitter::Query> =
            once_cell::sync::Lazy::new(|| {
                tree_sitter::Query::new(
                    tree_sitter_surrealql::language(),
                    r#"(
                        (keyword_select) @select
                        .
                        (_) @neighbor

                    )"#,
                )
                .expect("Could not initialise query")
            });

        for match_ in cursor.matches(&QUERY_SELECT_1, tree.root_node(), curr_doc) {
            let caps = match_.captures;
            if caps.len() < 2 {
                continue;
            }
            let select_range = caps[0].node.range();
            let neighbor_range = caps[1].node.range();
            if cursor_after(cursor_line, cursor_char, select_range.end_point)
                && cursor_before(cursor_line, cursor_char, neighbor_range.start_point)
            {
                return options_to_completions_map.get("select_options").cloned();
            }
        }

        // finally, we'll restrict our query to the cursor's current line and check
        // for a select without worrying about neighbors
        static QUERY_SELECT_2: once_cell::sync::Lazy<tree_sitter::Query> =
            once_cell::sync::Lazy::new(|| {
                tree_sitter::Query::new(
                    tree_sitter_surrealql::language(),
                    "(keyword_select) @select",
                )
                .expect("Could not initialise query")
            });

        // suggest * if cursor is past but on the same line
        cursor.set_point_range(std::ops::Range {
            start: tree_sitter::Point {
                row: cursor_line,
                column: 0,
            },
            end: tree_sitter::Point {
                row: cursor_line,
                column: usize::MAX,
            },
        });
        for match_ in cursor.matches(&QUERY_SELECT_2, tree.root_node(), curr_doc) {
            for capture in match_.captures.iter() {
                let arg_end = capture.node.range().end_point;
                if cursor_after(cursor_line, cursor_char, arg_end) {
                    return options_to_completions_map.get("select_options").cloned();
                }
            }
        }
    }
    None
}

struct Backend {
    client: tower_lsp::Client,
    parser: Arc<Mutex<tree_sitter::Parser>>,
    curr_doc: Arc<Mutex<Option<lsp_textdocument::FullTextDocument>>>,
    tree: Arc<Mutex<Option<tree_sitter::Tree>>>,
    completions_map:
        std::collections::HashMap<&'static str, Vec<tower_lsp::lsp_types::CompletionItem>>,
}

impl Backend {
    pub fn new(client: tower_lsp::Client) -> Self {
        Self {
            client,
            parser: Arc::new(Mutex::new(initialise_parser())),
            curr_doc: Arc::new(Mutex::new(None)),
            tree: Arc::new(Mutex::new(None)),
            completions_map: get_options_to_completion_items_map(),
        }
    }
}

#[tower_lsp::async_trait]
impl tower_lsp::LanguageServer for Backend {
    async fn initialize(
        &self,
        _: tower_lsp::lsp_types::InitializeParams,
    ) -> tower_lsp::jsonrpc::Result<tower_lsp::lsp_types::InitializeResult> {
        Ok(tower_lsp::lsp_types::InitializeResult {
            server_info: Some(tower_lsp::lsp_types::ServerInfo {
                name: String::from("surrealql-lsp"),
                version: Some(String::from("0.0.1")),
            }),
            capabilities: tower_lsp::lsp_types::ServerCapabilities {
                text_document_sync: Some(tower_lsp::lsp_types::TextDocumentSyncCapability::Kind(
                    tower_lsp::lsp_types::TextDocumentSyncKind::INCREMENTAL,
                )),
                completion_provider: Some(tower_lsp::lsp_types::CompletionOptions {
                    resolve_provider: Some(false),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    ..Default::default()
                }),
                ..tower_lsp::lsp_types::ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: tower_lsp::lsp_types::InitializedParams) {
        self.client
            .log_message(tower_lsp::lsp_types::MessageType::INFO, "initialized!")
            .await;
    }

    async fn shutdown(&self) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: tower_lsp::lsp_types::DidOpenTextDocumentParams) {
        let mut curr_doc = self.curr_doc.lock().await;
        let mut tree = self.tree.lock().await;
        let mut parser = self.parser.lock().await;

        *curr_doc = Some(lsp_textdocument::FullTextDocument::new(
            params.text_document.language_id.clone(),
            params.text_document.version,
            params.text_document.text.clone(),
        ));
        *tree = parser.parse(params.text_document.text, None);
    }

    async fn did_change(&self, params: tower_lsp::lsp_types::DidChangeTextDocumentParams) {
        let mut curr_doc = self.curr_doc.lock().await;
        let mut tree = self.tree.lock().await;

        if let Some(ref mut doc) = *curr_doc {
            doc.update(&params.content_changes, params.text_document.version);
            for change in params.content_changes.iter() {
                if let Some(ref mut curr_tree) = *tree {
                    match text_doc_change_to_tree_sitter_edit(change, doc) {
                        Ok(edit) => {
                            curr_tree.edit(&edit);
                        }
                        Err(err) => {
                            self.client
                                .log_message(
                                    tower_lsp::lsp_types::MessageType::ERROR,
                                    format!("Bad edit info, failed to edit tree: {}", err),
                                )
                                .await;
                        }
                    }
                }
            }
        }
    }

    async fn completion(
        &self,
        params: tower_lsp::lsp_types::CompletionParams,
    ) -> tower_lsp::jsonrpc::Result<Option<tower_lsp::lsp_types::CompletionResponse>> {
        let curr_doc = self.curr_doc.lock().await;
        let mut tree = self.tree.lock().await;
        let mut parser = self.parser.lock().await;

        if let Some(ref doc) = *curr_doc {
            let completion_list = get_completion_list(
                doc.get_content(None),
                &mut parser,
                &mut tree,
                &params,
                &self.completions_map,
            );
            match completion_list {
                Some(list) => {
                    return Ok(Some(tower_lsp::lsp_types::CompletionResponse::List(
                        tower_lsp::lsp_types::CompletionList {
                            is_incomplete: true,
                            items: list,
                        },
                    )))
                }
                _ => return Ok(None),
            }
        }

        Ok(None)
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());

    let (service, socket) = tower_lsp::LspService::build(Backend::new).finish();
    tower_lsp::Server::new(stdin, stdout, socket)
        .serve(service)
        .await;
}
