fn normalize_document_and_cursor_position(
    doc: &str,
    cursor_line: usize,
    cursor_char: usize,
) -> (String, usize, usize) {
    let lines: Vec<&str> = doc.lines().collect();
    if lines.len() <= 1 {
        return (doc.to_string(), cursor_line, cursor_char);
    }

    let content_before_cursor =
        lines.iter().take(cursor_line).copied().collect::<Vec<&str>>().join(" ");
    let new_cursor_char = content_before_cursor.len() + cursor_char + 1;
    let normalized_doc = format!("{}\n", lines.join(" "));

    (normalized_doc, 0, new_cursor_char)
}

fn cursor_matches(
    cursor_line: usize,
    cursor_char: usize,
    query_start: tree_sitter::Point,
    query_end: tree_sitter::Point,
) -> bool {
    // Completely envelop the cursor line-wise
    if query_start.row < cursor_line && query_end.row > cursor_line {
        return true;
    }

    // Single line match, check columns on both sides
    if cursor_line == query_start.row
        && cursor_line == query_end.row
        && query_start.column <= cursor_char
        && query_end.column >= cursor_char
    {
        return true;
    }

    // Start lines overlap, but the start column is before the cursor column
    if cursor_line == query_start.row && query_start.column <= cursor_char {
        return true;
    }

    // End lines overlap, but the end column is after the cursor column
    if cursor_line == query_end.row && query_end.column >= cursor_char {
        return true;
    }

    false
}

fn cursor_before(cursor_line: usize, cursor_char: usize, query_start: tree_sitter::Point) -> bool {
    cursor_line < query_start.row
        || (cursor_line == query_start.row && cursor_char < query_start.column)
}

fn cursor_after(cursor_line: usize, cursor_char: usize, query_end: tree_sitter::Point) -> bool {
    cursor_line > query_end.row || (cursor_line == query_end.row && cursor_char > query_end.column)
}

fn get_completion_for_context(
    cursor: &mut tree_sitter::QueryCursor,
    root_node: tree_sitter::Node,
    doc_bytes: &[u8],
    cursor_line: usize,
    cursor_char: usize,
) -> Option<Vec<String>> {
    static QUERY_CONTEXT: once_cell::sync::Lazy<tree_sitter::Query> =
        once_cell::sync::Lazy::new(|| {
            tree_sitter::Query::new(
                tree_sitter_surrealql::language(),
                r#"(from_clause (target) @target_options . ) (keyword_select) @select_options"#,
            )
            .expect("Could not initialize query")
        });

    static COMPLETION_CONTEXT_MAP: once_cell::sync::Lazy<
        std::collections::HashMap<String, Vec<String>>,
    > = once_cell::sync::Lazy::new(|| {
        let mut map: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();
        map.insert(
            "target_options".to_string(),
            vec![
                "WHERE".to_string(),
                "SPLIT".to_string(),
                "WITH".to_string(),
                "GROUP BY".to_string(),
                "LIMIT".to_string(),
                "ORDER BY".to_string(),
                "TIMEOUT".to_string(),
                "EXPLAIN".to_string(),
                "PARALLEL".to_string(),
            ],
        );
        map.insert("select_options".to_string(), vec!["VALUE".to_string()]);
        map
    });

    let mut last_match = None;
    for m in cursor.matches(&QUERY_CONTEXT, root_node, doc_bytes) {
        for capture in m.captures.iter() {
            let capture_name = &QUERY_CONTEXT.capture_names()[capture.index as usize];
            if cursor_matches(
                cursor_line,
                cursor_char,
                capture.node.range().start_point,
                capture.node.range().end_point,
            ) {
                last_match = Some((capture_name.clone(), capture.node.range()));
            }
        }
    }

    last_match.and_then(|(capture_name, _range)| {
        COMPLETION_CONTEXT_MAP.get(capture_name.as_str()).cloned()
    })
}

fn get_completion_for_errors(
    cursor: &mut tree_sitter::QueryCursor,
    root_node: tree_sitter::Node,
    doc_bytes: &[u8],
    cursor_line: usize,
    cursor_char: usize,
) -> Option<Vec<String>> {
    static QUERY_ERROR_START: once_cell::sync::Lazy<tree_sitter::Query> =
        once_cell::sync::Lazy::new(|| {
            tree_sitter::Query::new(tree_sitter_surrealql::language(), "(ERROR) @start")
                .expect("Could not initialize query")
        });

    for m in cursor.matches(&QUERY_ERROR_START, root_node, doc_bytes) {
        for capture in m.captures.iter() {
            if cursor_matches(
                cursor_line,
                cursor_char,
                capture.node.range().start_point,
                capture.node.range().end_point,
            ) {
                return Some(vec!["VALUE".to_string()]);
            }
        }
    }
    None
}

fn get_completion_for_select_neighbors(
    cursor: &mut tree_sitter::QueryCursor,
    root_node: tree_sitter::Node,
    doc_bytes: &[u8],
    cursor_line: usize,
    cursor_char: usize,
) -> Option<Vec<String>> {
    static QUERY_SELECT_NEIGHBOR: once_cell::sync::Lazy<tree_sitter::Query> =
        once_cell::sync::Lazy::new(|| {
            tree_sitter::Query::new(
                tree_sitter_surrealql::language(),
                r#"(
                    (keyword_select) @select
                    .
                    (_) @neighbor
                )"#,
            )
            .expect("Could not initialize query")
        });

    for m in cursor.matches(&QUERY_SELECT_NEIGHBOR, root_node, doc_bytes) {
        if m.captures.len() < 2 {
            continue;
        }
        let select_range = m.captures[0].node.range();
        let neighbor_range = m.captures[1].node.range();
        if cursor_after(cursor_line, cursor_char, select_range.end_point)
            && cursor_before(cursor_line, cursor_char, neighbor_range.start_point)
        {
            return Some(vec!["VALUE".to_string()]);
        }
    }
    None
}

fn get_completion_for_select(
    cursor: &mut tree_sitter::QueryCursor,
    root_node: tree_sitter::Node,
    doc_bytes: &[u8],
    cursor_line: usize,
    cursor_char: usize,
) -> Option<Vec<String>> {
    static QUERY_SELECT: once_cell::sync::Lazy<tree_sitter::Query> =
        once_cell::sync::Lazy::new(|| {
            tree_sitter::Query::new(tree_sitter_surrealql::language(), "(keyword_select) @select")
                .expect("Could not initialize query")
        });
    cursor.set_point_range(std::ops::Range {
        start: tree_sitter::Point { row: cursor_line, column: 0 },
        end: tree_sitter::Point { row: cursor_line, column: usize::MAX },
    });
    for m in cursor.matches(&QUERY_SELECT, root_node, doc_bytes) {
        for capture in m.captures {
            if cursor_after(cursor_line, cursor_char, capture.node.range().end_point) {
                return Some(vec!["VALUE".to_string()]);
            }
        }
    }
    None
}

pub(crate) fn get_completion_list(
    curr_doc: &str,
    parser: &mut tree_sitter::Parser,
    curr_tree: &mut Option<tree_sitter::Tree>,
    params: &tower_lsp::lsp_types::CompletionParams,
) -> Option<Vec<String>> {
    let cursor_line = params.text_document_position.position.line as usize;
    let cursor_char = params.text_document_position.position.character as usize;

    let (normalized_doc, cursor_line, cursor_char) =
        normalize_document_and_cursor_position(curr_doc, cursor_line, cursor_char);

    *curr_tree = parser.parse(&normalized_doc, curr_tree.as_ref());
    if let Some(tree) = curr_tree {
        let mut cursor = tree_sitter::QueryCursor::new();
        let doc_bytes = normalized_doc.as_bytes();
        let root_node = tree.root_node();

        get_completion_for_context(&mut cursor, root_node, doc_bytes, cursor_line, cursor_char)
            .or_else(|| {
                get_completion_for_errors(
                    &mut cursor,
                    root_node,
                    doc_bytes,
                    cursor_line,
                    cursor_char,
                )
            })
            .or_else(|| {
                get_completion_for_select_neighbors(
                    &mut cursor,
                    root_node,
                    doc_bytes,
                    cursor_line,
                    cursor_char,
                )
            })
            .or_else(|| {
                get_completion_for_select(
                    &mut cursor,
                    root_node,
                    doc_bytes,
                    cursor_line,
                    cursor_char,
                )
            })
    } else {
        None
    }
}
