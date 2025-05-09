use crate::{configs::*, operations::*};
use serde::Deserialize;
use std::error::Error;
use tauri::AppHandle;

// Available frontend-to-backend commands
#[derive(Deserialize, Clone)]
#[serde(tag = "name")]
pub enum Command {
    LoadSettings,
    GetDetails {
        path: ExePath,
    },
    Installation {
        config: InstallationConfig,
    },
    Uninstallation {
        timestamp: i64,
    },
    Elevate {
        revert: bool,
    },
    ValidatePath {
        path: String,
    },
    SaveSettings {
        settings: Settings,
    },
    LoadAppList,
    SaveAppList {
        app_list: AppList,
    },
    GetArchiveContent {
        path: String,
    },
    Open {
        path: String,
    },
    OpenFolder {
        path: String,
    },
    OpenRegistry {
        app_name: String,
        current_user_only: bool,
    },
    CheckPathEmpty {
        path: String,
    },
    Cli,
    RegisterContextMenu,
    UnregisterContextMenu,
    InstallWithLink {
        url: String,
        timestamp: i64,
    },
    SetStartup,
    RemoveStartup,
}

impl Command {
    // Routes command to appropriate handler function
    // Returns JSON-formatted response string or error message
    async fn execute(self, app: AppHandle) -> Result<String, Box<dyn Error + Send + Sync>> {
        match self {
            Self::LoadSettings => load_settings().await,
            Self::GetDetails { path } => get_details(path).await,
            Self::Installation { config } => installation(config, app).await,
            Self::Uninstallation { timestamp } => uninstallation(timestamp, app).await,
            Self::Elevate { revert } => elevate(revert).await,
            Self::ValidatePath { path } => validate_path(path).await,
            Self::SaveSettings { settings } => save_settings(settings).await,
            Self::LoadAppList => load_app_list().await,
            Self::SaveAppList { app_list } => save_app_list(app_list).await,
            Self::GetArchiveContent { path } => get_archive_content(path).await,
            Self::Open { path } => open_app(&path).await,
            Self::OpenFolder { path } => open_folder(&path).await,
            Self::OpenRegistry {
                app_name,
                current_user_only,
            } => open_registry(&app_name, current_user_only).await,
            Self::CheckPathEmpty { path } => check_path_empty(&path).await,
            Self::Cli => cli(app).await,
            Self::RegisterContextMenu => register_context_menu(),
            Self::UnregisterContextMenu => unregister_context_menu(),
            Self::InstallWithLink { url, timestamp } => install_with_link(url, timestamp).await,
            Self::SetStartup => set_startup(),
            Self::RemoveStartup => remove_startup(),
        }
    }
}

// Frontend-to-backend command bridge
#[tauri::command(async)]
pub async fn execute_command(command: Command, app: AppHandle) -> Result<String, String> {
    command.execute(app).await.map_err(|e| e.to_string())
}
