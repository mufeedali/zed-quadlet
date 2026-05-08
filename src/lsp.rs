use std::fs;

use zed_extension_api::{
    self as zed, GithubReleaseOptions, LanguageServerId, Result, Worktree, settings::LspSettings,
};

pub(crate) struct QuadletLsp {
    cached_binary_path: Option<String>,
}

impl QuadletLsp {
    pub(crate) const LANGUAGE_SERVER_ID: &str = "quadlet-lsp";

    pub(crate) fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    pub(crate) fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let binary_path = self.language_server_binary(language_server_id, worktree)?;
        Ok(zed::Command {
            command: binary_path,
            args: vec![],
            env: worktree.shell_env(),
        })
    }

    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<String> {
        if let Some(path) = LspSettings::for_worktree(Self::LANGUAGE_SERVER_ID, worktree)
            .ok()
            .and_then(|settings| settings.binary)
            .and_then(|b| b.path)
        {
            return Ok(path);
        }

        if let Some(path) = worktree.which(Self::LANGUAGE_SERVER_ID) {
            return Ok(path);
        }

        if let Some(path) = &self.cached_binary_path
            && fs::metadata(path).is_ok_and(|m| m.is_file())
        {
            return Ok(path.clone());
        }

        let (platform, arch) = zed::current_platform();
        let binary_name = match platform {
            zed::Os::Windows => "quadlet-lsp.exe",
            _ => "quadlet-lsp",
        };

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = match zed::latest_github_release(
            "onlyati/quadlet-lsp",
            GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        ) {
            Ok(release) => release,
            Err(_) => {
                if let Some(path) = Self::find_existing_binary(binary_name) {
                    self.cached_binary_path = Some(path.clone());
                    return Ok(path);
                }
                return Err("failed to fetch latest GitHub release".into());
            }
        };

        let os = match platform {
            zed::Os::Mac => "darwin",
            zed::Os::Linux => "linux",
            zed::Os::Windows => "windows",
        };

        let arch_str = match arch {
            zed::Architecture::Aarch64 => "arm64",
            zed::Architecture::X8664 => "amd64",
            zed::Architecture::X86 => return Err("x86 architecture is not supported".into()),
        };

        let (ext, download_format) = match platform {
            zed::Os::Mac | zed::Os::Linux => ("tar.gz", zed::DownloadedFileType::GzipTar),
            zed::Os::Windows => ("zip", zed::DownloadedFileType::Zip),
        };

        let version = release
            .version
            .strip_prefix('v')
            .unwrap_or(&release.version);
        let asset_name = format!("quadlet-lsp-{version}-{os}-{arch_str}.{ext}");

        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| format!("no release asset found matching {asset_name}"))?;

        let version_dir = format!("quadlet-lsp-{version}");
        let binary_path = format!("{}/{}", version_dir, binary_name);

        if !fs::metadata(&binary_path).is_ok_and(|m| m.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(&asset.download_url, &version_dir, download_format)
                .map_err(|e| format!("failed to download quadlet-lsp: {e}"))?;

            zed::make_file_executable(&binary_path)?;

            Self::remove_outdated_versions(&version_dir)?;
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }

    fn remove_outdated_versions(version_dir: &str) -> Result<()> {
        let entries = fs::read_dir(".").map_err(|e| format!("failed to list directory: {e}"))?;
        for entry in entries {
            let entry = entry.map_err(|e| format!("failed to read directory entry: {e}"))?;
            if entry.file_name().to_str().is_none_or(|name| {
                name.starts_with(Self::LANGUAGE_SERVER_ID) && name != version_dir
            }) {
                fs::remove_dir_all(entry.path()).ok();
            }
        }
        Ok(())
    }

    fn find_existing_binary(binary_name: &str) -> Option<String> {
        fs::read_dir(".").ok()?.flatten().find_map(|entry| {
            let binary_path = entry.path().join(binary_name);
            if binary_path.is_file()
                && entry
                    .file_name()
                    .to_str()
                    .is_some_and(|name| name.starts_with(Self::LANGUAGE_SERVER_ID))
            {
                Some(binary_path.to_string_lossy().to_string())
            } else {
                None
            }
        })
    }
}
