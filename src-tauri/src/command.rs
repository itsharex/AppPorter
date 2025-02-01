use crate::{elevate, operations::*, settings::load_settings, validate_path};
use serde::Deserialize;
use tauri::AppHandle;

/// Represents all available commands that can be invoked through Tauri
#[derive(Deserialize)]
#[serde(tag = "name")]
pub enum Command {
    LoadSettings,
    GetDetails { path: ExePath },
    Installation { config: InstallationConfig },
    Elevate { revert: bool },
    ValidatePath { path: String },
}

impl Command {
    async fn execute(self, app: AppHandle) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Self::LoadSettings => load_settings().await,
            Self::GetDetails { path } => get_details(path, app).await,
            Self::Installation { config } => installation(config, app).await,
            Self::Elevate { revert } => elevate(revert).await.map(|_| "Success".to_string()),
            Self::ValidatePath { path } => validate_path(path).await,
        }
    }
}

/// Executes a command and returns the result as a string
#[tauri::command(async)]
pub async fn execute_command(command: Command, app: AppHandle) -> Result<String, String> {
    command.execute(app).await.map_err(|e| e.to_string())
}
