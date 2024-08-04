pub(crate) fn initialise_parser() -> tree_sitter::Parser {
    let mut parser = tree_sitter::Parser::new();
    if parser.set_language(tree_sitter_surrealql::language()).is_err() {
        panic!("Failed to set parser language");
    }
    parser
}
