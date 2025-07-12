//! Language Server Protocol implementation for the Gigli programming language

use anyhow::Result;
use tower_lsp::{LspService, Server};

mod lsp;

#[tokio::main]
async fn main() -> Result<()> {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(lsp::GigliLanguageServer::new);
    
    Server::new(stdin, stdout, socket)
        .serve(service)
        .await;

    Ok(())
}
