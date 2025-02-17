use crate::configs::ConfigFile;
use check_elevation::is_elevated;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::process::Command;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Settings {
    // UI and preferences
    #[serde(default)]
    pub language: String,
    #[serde(default)]
    pub theme: String,
    #[serde(default)]
    pub minimize_to_tray_on_close: bool,
    #[serde(default)]
    pub color: String,

    // System information
    #[serde(default)]
    pub debug: bool,
    #[serde(default)]
    pub elevated: bool,
    #[serde(default)]
    pub run_as_admin: bool,
    #[serde(default)]
    pub system_drive_letter: String,
    #[serde(default)]
    pub user_sid: String,
    #[serde(default)]
    pub username: String,

    #[serde(default)]
    pub installation: Installation,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Installation {
    #[serde(default)]
    pub current_user_only: bool,

    #[serde(default)]
    pub all_users: InstallSettings,
    #[serde(default)]
    pub current_user: InstallSettings,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct InstallSettings {
    #[serde(default)]
    pub create_desktop_shortcut: bool,
    #[serde(default)]
    pub create_registry_key: bool,
    #[serde(default)]
    pub create_start_menu_shortcut: bool,
    #[serde(default)]
    pub install_path: String,
}

#[async_trait::async_trait]
impl ConfigFile for Settings {
    fn get_filename() -> &'static str {
        "Settings.toml"
    }

    async fn create_default() -> Result<Self, Box<dyn Error>> {
        let default_settings = Self {
            language: String::from("en"),
            theme: String::from("system"),
            minimize_to_tray_on_close: false,
            color: String::new(),
            debug: cfg!(debug_assertions),
            elevated: is_elevated()?,
            run_as_admin: false,
            system_drive_letter: std::env::var("windir")?[..1].to_string(),
            user_sid: String::new(),
            username: std::env::var("USERNAME")?,

            installation: Installation {
                current_user_only: false,
                all_users: InstallSettings {
                    create_desktop_shortcut: false,
                    create_registry_key: true,
                    create_start_menu_shortcut: true,
                    install_path: format!(
                        r"{}:\Program Files",
                        std::env::var("windir")?[..1].to_string()
                    ),
                },
                current_user: InstallSettings {
                    create_desktop_shortcut: false,
                    create_registry_key: true,
                    create_start_menu_shortcut: true,
                    install_path: format!(
                        r"{}:\Users\{}\AppData\Local\Programs",
                        std::env::var("windir")?[..1].to_string(),
                        std::env::var("USERNAME")?
                    ),
                },
            },
        };

        let config_path = Self::get_file_path().await?;

        if let Some(parent) = config_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let content = tokio::task::spawn_blocking({
            let config = default_settings.clone();
            move || toml::to_string_pretty(&config)
        })
        .await??;

        tokio::fs::write(config_path, content).await?;
        Ok(default_settings)
    }
}

impl Settings {
    pub async fn initialization(&mut self) -> Result<(), Box<dyn Error>> {
        self.debug = cfg!(debug_assertions);
        self.elevated = is_elevated()?;
        self.user_sid = self.get_user_sid().await?;
        self.run_as_admin = self.check_run_as_admin()?;
        self.update_system_info()?;
        self.update_color_settings().await?;
        self.update_install_paths();
        self.save().await?;
        Ok(())
    }

    async fn get_user_sid(&self) -> Result<String, Box<dyn Error>> {
        let output = Command::new("powershell")
            .args([
                "-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass",
                "-Command",
                r#"Get-WmiObject Win32_UserAccount -Filter "Name='$env:USERNAME'" | Select-Object -ExpandProperty SID"#,
            ]).creation_flags(0x08000000)
            .output()
            .await?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into());
        }
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    fn check_run_as_admin(&self) -> Result<bool, Box<dyn Error>> {
        Ok(windows_registry::USERS
            .open(format!(
                r"{}\Software\Microsoft\Windows NT\CurrentVersion\AppCompatFlags\Layers",
                self.user_sid
            ))?
            .get_string(std::env::current_exe()?.to_string_lossy().to_string())
            .map(|value| value.contains("RUNASADMIN"))
            .unwrap_or(false))
    }

    fn update_system_info(&mut self) -> Result<(), Box<dyn Error>> {
        self.system_drive_letter = std::env::var("windir")?[..1].to_string();
        self.username = std::env::var("USERNAME")?.to_string();
        Ok(())
    }

    async fn update_color_settings(&mut self) -> Result<(), Box<dyn Error>> {
        let accent_color = windows_registry::CURRENT_USER
            .open(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Accent")?
            .get_u32("AccentColorMenu")?;
        let accent_color_str = format!("{:x}", accent_color);
        let (b, g, r) = (
            &accent_color_str[2..4],
            &accent_color_str[4..6],
            &accent_color_str[6..8],
        );
        self.color = format!("#{}{}{}", r, g, b);
        Ok(())
    }

    fn update_install_paths(&mut self) {
        if self.installation.all_users.install_path.is_empty() {
            self.installation.all_users.install_path =
                format!(r"{}:\Program Files", self.system_drive_letter);
        }

        if self.installation.current_user.install_path.is_empty() {
            self.installation.current_user.install_path = format!(
                r"{}:\Users\{}\AppData\Local\Programs",
                self.system_drive_letter, self.username
            );
        }
    }
}

pub async fn load_settings() -> Result<String, Box<dyn Error>> {
    let mut settings = Settings::read().await?;
    settings.initialization().await?;
    Ok(serde_json::to_string(&settings)?)
}

pub async fn save_settings(settings: Settings) -> Result<String, Box<dyn Error>> {
    settings.save().await?;
    Ok("Settings saved successfully".to_string())
}
