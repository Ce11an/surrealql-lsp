mod completion;
mod keywords;
mod lsp;
mod parser;
mod position;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());

    let (service, socket) = tower_lsp::LspService::build(lsp::Backend::new).finish();
    tower_lsp::Server::new(stdin, stdout, socket).serve(service).await;
}
