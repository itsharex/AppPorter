use super::super::settings::*;
use mslnk::ShellLink;
use std::{error::Error, fs::File, path::Path, result::Result};
use zip::read::ZipArchive;
use zip_extensions::read::ZipArchiveExtensions;

fn installation(zip_path: String, settings: Settings) -> Result<(), Box<dyn Error>> {
    let specific_settings = match settings.installation.install_mode {
        InstallMode::AllUsers => settings.installation.all_users,
        InstallMode::CurrentUser => settings.installation.current_user,
    };

    let file = File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    archive.extract(specific_settings.install_path)?;

    if specific_settings.create_registry_key {
        let key_path = format!(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\{}",
            r"_"
        );

        let key = windows_registry::LOCAL_MACHINE.create(key_path)?;

        if specific_settings.registry.create_comments {
            key.set_string("Comments", "")?;
        }
        if specific_settings.registry.create_display_icon {
            key.set_string("DisplayIcon", "")?;
        }
        if specific_settings.registry.create_display_name {
            key.set_string("DisplayName", "")?;
        }
        if specific_settings.registry.create_display_version {
            key.set_string("DisplayVersion", "")?;
        }
        if specific_settings.registry.create_estimated_size {
            key.set_u32("EstimatedSize", 0)?;
        }
        if specific_settings.registry.create_install_location {
            key.set_string("InstallLocation", "")?;
        }
        if specific_settings.registry.create_no_modify {
            key.set_u32("NoModify", 0)?;
        }
        if specific_settings.registry.create_no_remove {
            key.set_u32("NoRemove", 0)?;
        }
        if specific_settings.registry.create_no_repair {
            key.set_u32("NoRepair", 0)?;
        }
        if specific_settings.registry.create_publisher {
            key.set_string("Publisher", "")?;
        }
        if specific_settings.registry.create_uninstall_string {
            key.set_string("UninstallString", "")?;
        }
    }
    if specific_settings.create_start_menu_shortcut
        && matches!(settings.installation.install_mode, InstallMode::AllUsers)
    {
        ShellLink::new(r"")?.create_lnk(r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs")?;
    }

    if specific_settings.create_start_menu_shortcut
        && matches!(settings.installation.install_mode, InstallMode::CurrentUser)
    {
        ShellLink::new(r"")?
            .create_lnk(r"C:\Users\_\AppData\Roaming\Microsoft\Windows\Start Menu\Programs")?;
    }

    if specific_settings.create_desktop_shortcut {
        ShellLink::new(r"")?.create_lnk(r"")?;
    }

    Ok(())
}
