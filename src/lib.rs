use zed_extension_api::{self as zed, LanguageServerId, Result};

const QUADLET_LSP_VERSION: &str = "0.7.3";

struct QuadletExtension;

impl zed::Extension for QuadletExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            "quadlet-lsp" => Ok(zed::Command {
                command: self.language_server_binary_path(language_server_id, worktree)?,
                args: vec![],
                env: Default::default(),
            }),
            id => Err(format!("unknown language server: {id}")),
        }
    }
}

impl QuadletExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which("quadlet-lsp") {
            return Ok(path);
        }

        let (platform, arch) = zed::current_platform();

        let (os, download_format, zed_format, binary_name) = match platform {
            zed::Os::Mac => (
                "darwin",
                "tar.gz",
                zed::DownloadedFileType::GzipTar,
                "quadlet-lsp",
            ),
            zed::Os::Linux => (
                "linux",
                "tar.gz",
                zed::DownloadedFileType::GzipTar,
                "quadlet-lsp",
            ),
            zed::Os::Windows => (
                "windows",
                "zip",
                zed::DownloadedFileType::Zip,
                "quadlet-lsp.exe",
            ),
        };

        let arch = match arch {
            zed::Architecture::Aarch64 => "arm64",
            zed::Architecture::X8664 => "amd64",
            zed::Architecture::X86 => {
                return Err("quadlet-lsp does not support x86 architecture".into());
            }
        };

        let version_dir = format!("quadlet-lsp-{QUADLET_LSP_VERSION}");
        let binary_path = format!("{version_dir}/{binary_name}");

        if std::fs::metadata(&binary_path).is_ok_and(|m| m.is_file()) {
            return Ok(binary_path);
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );

        let download_url = format!(
            "https://github.com/onlyati/quadlet-lsp/releases/download/v{QUADLET_LSP_VERSION}/quadlet-lsp-{QUADLET_LSP_VERSION}-{os}-{arch}.{download_format}"
        );

        zed::download_file(&download_url, &version_dir, zed_format)
            .map_err(|e| format!("failed to download quadlet-lsp: {e}"))?;

        zed::make_file_executable(&binary_path)?;

        Ok(binary_path)
    }
}

zed::register_extension!(QuadletExtension);
