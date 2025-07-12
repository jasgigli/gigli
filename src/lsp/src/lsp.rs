//! Core LSP server implementation

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

pub struct GigliLanguageServer {
    client: Client,
}

impl GigliLanguageServer {
    pub fn new(client: Client) -> Self {
        GigliLanguageServer { client }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for GigliLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Gigli Language Server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, _: DidOpenTextDocumentParams) {
        // TODO: Implement document parsing and analysis
    }

    async fn did_change(&self, _: DidChangeTextDocumentParams) {
        // TODO: Implement incremental parsing and analysis
    }

    async fn did_close(&self, _: DidCloseTextDocumentParams) {
        // TODO: Clean up document resources
    }
}
