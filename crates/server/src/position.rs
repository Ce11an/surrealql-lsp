pub(crate) fn retrieve_keyword_at_position(
    document_content: &str,
    parser: &mut tree_sitter::Parser,
    syntax_tree: &mut Option<tree_sitter::Tree>,
    cursor_line: usize,
    cursor_character: usize,
) -> Option<String> {
    *syntax_tree = parser.parse(document_content, syntax_tree.as_ref());
    let tree = syntax_tree.as_ref()?;

    let mut query_cursor = tree_sitter::QueryCursor::new();
    let document_bytes = document_content.as_bytes();

    static KEYWORD_QUERY: once_cell::sync::Lazy<tree_sitter::Query> =
        once_cell::sync::Lazy::new(|| {
            tree_sitter::Query::new(
                tree_sitter_surrealql::language(),
                r#"
            [
             (keyword_explain)
             (keyword_from)
             (keyword_group_by)
             (keyword_limit)
             (keyword_only)
             (keyword_order_by)
             (keyword_parallel)
             (keyword_select)
             (keyword_split)
             (keyword_timeout)
             (keyword_value)
             (keyword_where)
             (keyword_with)
            ] @keywords
            "#,
            )
            .expect("Failed to create keyword query")
        });

    find_keyword_at_position(
        &mut query_cursor,
        &KEYWORD_QUERY,
        tree.root_node(),
        document_bytes,
        cursor_line,
        cursor_character,
    )
}

fn find_keyword_at_position(
    query_cursor: &mut tree_sitter::QueryCursor,
    query: &tree_sitter::Query,
    root_node: tree_sitter::Node,
    document_bytes: &[u8],
    cursor_line: usize,
    cursor_character: usize,
) -> Option<String> {
    for match_ in query_cursor.matches(query, root_node, document_bytes) {
        for capture in match_.captures {
            let node = capture.node;
            let start_position = node.start_position();
            let end_position = node.end_position();

            if is_within_cursor_range(start_position, end_position, cursor_line, cursor_character) {
                return node.utf8_text(document_bytes).ok().map(String::from);
            }
        }
    }
    None
}

fn is_within_cursor_range(
    start_position: tree_sitter::Point,
    end_position: tree_sitter::Point,
    cursor_line: usize,
    cursor_character: usize,
) -> bool {
    start_position.row == cursor_line
        && end_position.row == cursor_line
        && start_position.column <= cursor_character
        && end_position.column >= cursor_character
}
