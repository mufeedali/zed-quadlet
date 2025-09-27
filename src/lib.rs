use zed_extension_api::{self as zed, Result};

struct QuadletExtension;

impl zed::Extension for QuadletExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            "quadlet-lsp" => {
                // Try to find quadlet-lsp in PATH first
                if let Some(path) = worktree.which("quadlet-lsp") {
                    return Ok(zed::Command {
                        command: path,
                        args: vec![],
                        env: Default::default(),
                    });
                }

                // If not found, try to download and install it
                self.download_quadlet_lsp()
            }
            language_server_id => Err(format!("unknown language server: {language_server_id}"))?,
        }
    }
}

impl QuadletExtension {
    fn download_quadlet_lsp(&mut self) -> Result<zed::Command> {
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
            // x86 is not supported by `quadlet-lsp`
            zed::Architecture::X86 => {
                return Err("quadlet-lsp does not support x86 architecture".into());
            }
        };

        let version = "0.5.0";
        let download_url = format!(
            "https://github.com/onlyati/quadlet-lsp/releases/download/v{}/quadlet-lsp-{}-{}-{}.{}",
            version, version, os, arch, download_format
        );

        let version_dir = format!("quadlet-lsp-{}", version);
        let binary_path = format!("{}/{}", version_dir, binary_name);

        zed::download_file(&download_url, &version_dir, zed_format)
            .map_err(|e| format!("Failed to download quadlet-lsp: {e}"))?;

        // Make the binary executable
        zed::make_file_executable(&binary_path)?;

        Ok(zed::Command {
            command: binary_path,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(QuadletExtension);
