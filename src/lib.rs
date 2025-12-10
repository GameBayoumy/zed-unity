use std::fs;
use zed_extension_api::{
    self as zed, settings::LspSettings, Command, DownloadedFileType, LanguageServerId,
    Os, Result, Worktree,
};

/// Zed Unity Extension
/// Provides comprehensive Unity development support:
/// - Roslyn-based C# language server (csharp-language-server)
/// - USS (Unity Style Sheets) language server
/// - Unity debugging via DAP (Mono debugger)
struct UnityExtension {
    cached_csharp_binary_path: Option<String>,
    cached_uss_binary_path: Option<String>,
    cached_netcoredbg_path: Option<String>,
}

impl UnityExtension {
    /// Get the appropriate USS language server release asset name for the current platform
    fn get_uss_server_asset_name(&self) -> Result<String, String> {
        let (os, arch) = zed::current_platform();

        let platform = match os {
            Os::Linux => match arch {
                zed::Architecture::Aarch64 => "linux-arm64",
                zed::Architecture::X8664 => "linux-x64",
                _ => return Err(format!("Unsupported Linux architecture: {:?}", arch)),
            },
            Os::Mac => match arch {
                zed::Architecture::Aarch64 => "darwin-arm64",
                zed::Architecture::X8664 => "darwin-x64",
                _ => return Err(format!("Unsupported macOS architecture: {:?}", arch)),
            },
            Os::Windows => match arch {
                zed::Architecture::X8664 => "win-x64",
                _ => return Err(format!("Unsupported Windows architecture: {:?}", arch)),
            },
            _ => return Err(format!("Unsupported operating system: {:?}", os)),
        };

        let (os_type, _) = zed::current_platform();
        let ext = match os_type {
            Os::Windows => "zip",
            _ => "tar.gz",
        };

        Ok(format!("uss-language-server-{}.{}", platform, ext))
    }

    /// Get the appropriate csharp-language-server release asset name for the current platform
    fn get_csharp_server_asset_name(&self) -> Result<String, String> {
        let (os, arch) = zed::current_platform();

        let platform = match os {
            Os::Linux => match arch {
                zed::Architecture::Aarch64 => "linux-arm64",
                zed::Architecture::X8664 => "linux-x64",
                _ => return Err(format!("Unsupported Linux architecture: {:?}", arch)),
            },
            Os::Mac => match arch {
                zed::Architecture::Aarch64 => "osx-arm64",
                zed::Architecture::X8664 => "osx-x64",
                _ => return Err(format!("Unsupported macOS architecture: {:?}", arch)),
            },
            Os::Windows => match arch {
                zed::Architecture::Aarch64 => "win-arm64",
                zed::Architecture::X8664 => "win-x64",
                _ => return Err(format!("Unsupported Windows architecture: {:?}", arch)),
            },
            _ => return Err(format!("Unsupported operating system: {:?}", os)),
        };

        Ok(format!("csharp-language-server-{}.tar.gz", platform))
    }

    /// Get the appropriate netcoredbg release asset name for the current platform
    fn get_netcoredbg_asset_name(&self) -> Result<String, String> {
        let (os, arch) = zed::current_platform();

        let platform = match os {
            Os::Linux => match arch {
                zed::Architecture::Aarch64 => "linux-arm64",
                zed::Architecture::X8664 => "linux-amd64",
                _ => return Err(format!("Unsupported Linux architecture: {:?}", arch)),
            },
            Os::Mac => match arch {
                zed::Architecture::Aarch64 => "osx-arm64",
                zed::Architecture::X8664 => "osx-amd64",
                _ => return Err(format!("Unsupported macOS architecture: {:?}", arch)),
            },
            Os::Windows => match arch {
                zed::Architecture::X8664 => "win64",
                _ => return Err(format!("Unsupported Windows architecture: {:?}", arch)),
            },
            _ => return Err(format!("Unsupported operating system: {:?}", os)),
        };

        Ok(format!("netcoredbg-{}.tar.gz", platform))
    }

    /// Download and install csharp-language-server
    fn install_csharp_language_server(&self) -> Result<String, String> {
        let asset_name = self.get_csharp_server_asset_name()?;

        // Get the latest release from GitHub
        let release = zed::latest_github_release(
            "SofusA/csharp-language-server",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| format!("No asset found for platform: {}", asset_name))?;

        let version_dir = format!("csharp-language-server-{}", release.version);
        let binary_path = format!("{}/csharp-language-server", version_dir);

        // Check if already downloaded
        if fs::metadata(&binary_path).is_ok() {
            return Ok(binary_path);
        }

        // Download and extract
        zed::download_file(
            &asset.download_url,
            &version_dir,
            DownloadedFileType::GzipTar,
        )
        .map_err(|e| format!("Failed to download csharp-language-server: {}", e))?;

        // Make executable on Unix systems (permission is set by extraction)
        // The tar.gz should preserve file permissions

        Ok(binary_path)
    }

    /// Download and install netcoredbg (Debug Adapter for .NET/Mono)
    fn install_netcoredbg(&self) -> Result<String, String> {
        let asset_name = self.get_netcoredbg_asset_name()?;

        // Get the latest release from GitHub
        let release = zed::latest_github_release(
            "Samsung/netcoredbg",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let asset = release
            .assets
            .iter()
            .find(|a| a.name.contains(&asset_name.replace(".tar.gz", "")))
            .ok_or_else(|| format!("No asset found for platform: {}", asset_name))?;

        let version_dir = format!("netcoredbg-{}", release.version);
        
        let (os, _) = zed::current_platform();
        let binary_name = match os {
            Os::Windows => "netcoredbg.exe",
            _ => "netcoredbg",
        };
        let binary_path = format!("{}/netcoredbg/{}", version_dir, binary_name);

        // Check if already downloaded
        if fs::metadata(&binary_path).is_ok() {
            return Ok(binary_path);
        }

        // Download and extract
        zed::download_file(
            &asset.download_url,
            &version_dir,
            DownloadedFileType::GzipTar,
        )
        .map_err(|e| format!("Failed to download netcoredbg: {}", e))?;

        // Make executable on Unix systems (permission is set by extraction)
        // The tar.gz should preserve file permissions

        Ok(binary_path)
    }

    /// Get command for csharp-language-server
    fn csharp_language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        // Check for user-provided binary path in settings
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        let binary_path = if let Some(binary) = settings.binary.as_ref() {
            binary
                .path
                .clone()
                .ok_or_else(|| "Binary path not specified in settings".to_string())?
        } else if let Some(path) = &self.cached_csharp_binary_path {
            path.clone()
        } else {
            let path = self.install_csharp_language_server()?;
            self.cached_csharp_binary_path = Some(path.clone());
            path
        };

        // Get user-provided arguments or use defaults
        let args = settings
            .binary
            .as_ref()
            .and_then(|b| b.arguments.clone())
            .unwrap_or_default();

        Ok(Command {
            command: binary_path,
            args,
            env: Default::default(),
        })
    }

    /// Download and install USS language server from GitHub releases
    fn install_uss_language_server(&self, language_server_id: &LanguageServerId) -> Result<String, String> {
        let asset_name = self.get_uss_server_asset_name()?;

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        // Get the latest release from GitHub
        let release = zed::latest_github_release(
            "GameBayoumy/uss-language-server",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        ).map_err(|e| format!("Failed to fetch USS language server releases: {}", e))?;

        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| format!("No USS language server asset found for platform: {}", asset_name))?;

        let (os, _) = zed::current_platform();
        let binary_name = match os {
            Os::Windows => "uss-language-server.exe",
            _ => "uss-language-server",
        };

        let version_dir = format!("uss-language-server-{}", release.version);
        let binary_path = format!("{}/{}", version_dir, binary_name);

        // Check if already downloaded
        if fs::metadata(&binary_path).is_ok() {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::None,
            );
            return Ok(binary_path);
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );

        // Download and extract
        let download_type = match os {
            Os::Windows => DownloadedFileType::Zip,
            _ => DownloadedFileType::GzipTar,
        };

        zed::download_file(&asset.download_url, &version_dir, download_type)
            .map_err(|e| format!("Failed to download uss-language-server: {}", e))?;

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::None,
        );

        Ok(binary_path)
    }

    /// Get command for USS language server (downloads from GitHub releases)
    fn uss_language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let (os, _) = zed::current_platform();
        
        let binary_name = match os {
            Os::Windows => "uss-language-server.exe",
            _ => "uss-language-server",
        };

        // Check for user-provided binary path in settings
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        let binary_path = if let Some(binary) = settings.binary.as_ref() {
            binary
                .path
                .clone()
                .ok_or_else(|| "Binary path not specified in settings".to_string())?
        } else if let Some(path) = worktree.which(binary_name) {
            // Try to find uss-language-server in PATH
            path
        } else if let Some(path) = &self.cached_uss_binary_path {
            path.clone()
        } else {
            // Download from GitHub releases
            let path = self.install_uss_language_server(language_server_id)?;
            self.cached_uss_binary_path = Some(path.clone());
            path
        };

        // Get user-provided arguments or use defaults
        let args = settings
            .binary
            .as_ref()
            .and_then(|b| b.arguments.clone())
            .unwrap_or_default();

        Ok(Command {
            command: binary_path,
            args,
            env: Default::default(),
        })
    }

    /// Get debug adapter binary for Unity debugging
    fn get_unity_debugger(&mut self) -> Result<String, String> {
        if let Some(path) = &self.cached_netcoredbg_path {
            return Ok(path.clone());
        }

        let path = self.install_netcoredbg()?;
        self.cached_netcoredbg_path = Some(path.clone());
        Ok(path)
    }
}

impl zed::Extension for UnityExtension {
    fn new() -> Self {
        Self {
            cached_csharp_binary_path: None,
            cached_uss_binary_path: None,
            cached_netcoredbg_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        match language_server_id.as_ref() {
            "csharp-language-server" => {
                self.csharp_language_server_command(language_server_id, worktree)
            }
            "uss-language-server" => {
                self.uss_language_server_command(language_server_id, worktree)
            }
            _ => Err(format!("Unknown language server: {}", language_server_id.as_ref())),
        }
    }
}

zed::register_extension!(UnityExtension);
