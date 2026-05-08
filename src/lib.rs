mod lsp;

use zed_extension_api::{self as zed, LanguageServerId, Result, Worktree};

use crate::lsp::QuadletLsp;

struct QuadletExtension {
    lsp: QuadletLsp,
}

impl zed::Extension for QuadletExtension {
    fn new() -> Self {
        Self {
            lsp: QuadletLsp::new(),
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        if language_server_id.as_ref() != QuadletLsp::LANGUAGE_SERVER_ID {
            return Err(format!("unknown language server: {language_server_id}"));
        }
        self.lsp
            .language_server_command(language_server_id, worktree)
    }
}

zed::register_extension!(QuadletExtension);
