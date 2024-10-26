use crate::settings;
use ccv_contract::{
    app_error,
    error::AppError,
    models::{AllShortcuts, EventPayload, Settings, Shortcut, Theme},
};
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};
use tauri::{AppHandle, Emitter, Manager};

const SETTINGS_FILE_NAME: &str = "settings.json";
const DEFAULT_VERSION: &str = "v1";
const NOTIFICATIONS_KEY: &str = "notifications";
const ALL_SHORTCUTS_KEY: &str = "allShortcuts";
const OPEN_CCV_KEY: &str = "openCcv";
const VERSION_KEY: &str = "version";
const THEME_KEY: &str = "theme";
const AUTOSTART_KEY: &str = "autostart";

pub fn read_settings(app_data_dir: &PathBuf) -> Result<Settings, AppError> {
    let mut file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(app_data_dir.join(SETTINGS_FILE_NAME))
        .map_err(|err| app_error!("Unable to open settings file. {err}"))?;

    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .map_err(|err| app_error!("Unable to read settings file. {err}"))?;

    let settings_to_be_written = match serde_json::from_str::<serde_json::Value>(&file_content) {
        Ok(value) => fix_settings(value.clone(), get_default_settings()),
        Err(_) => get_default_settings(),
    };

    write_settings(app_data_dir, &settings_to_be_written)?;
    Ok(settings_to_be_written)
}

fn fix_settings(mut value: serde_json::Value, default_settings: Settings) -> Settings {
    if !value.is_object() {
        return default_settings;
    }

    match value[VERSION_KEY].as_str() {
        Some(version) => {
            match version {
                DEFAULT_VERSION => {}
                "v0" => {
                    value[NOTIFICATIONS_KEY] =
                        serde_json::json!(vec![settings::WELCOME_NOTIFICATION]);
                }
                _ => {
                    value[NOTIFICATIONS_KEY] = serde_json::json!(default_settings.notifications);
                }
            };
        }
        None => {
            value[NOTIFICATIONS_KEY] = serde_json::json!(default_settings.notifications);
        }
    };
    value[VERSION_KEY] = serde_json::json!(default_settings.version);

    match serde_json::from_value::<Vec<String>>(value[NOTIFICATIONS_KEY].clone()) {
        Err(_) => {
            value[NOTIFICATIONS_KEY] = serde_json::Value::Null;
        }
        Ok(mut notifications) => {
            if let Some(occupied_shortcut_index) = notifications
                .iter()
                .position(|n| n.as_str() == settings::OCCUPIED_SHORTCUT_NOTIFICATION)
            {
                notifications.remove(occupied_shortcut_index);
                value[NOTIFICATIONS_KEY] = serde_json::json!(notifications);
            }
        }
    }

    if let Err(_) = serde_json::from_value::<Theme>(value[THEME_KEY].clone()) {
        value[THEME_KEY] = serde_json::json!(default_settings.theme);
    }

    if let Err(_) = serde_json::from_value::<bool>(value[AUTOSTART_KEY].clone()) {
        value[AUTOSTART_KEY] = serde_json::json!(true);
    }

    if !value[ALL_SHORTCUTS_KEY].is_object() {
        value[ALL_SHORTCUTS_KEY] = serde_json::json!(default_settings.all_shortcuts);
    } else {
        if let Err(_) =
            serde_json::from_value::<Shortcut>(value[ALL_SHORTCUTS_KEY][OPEN_CCV_KEY].clone())
        {
            value[ALL_SHORTCUTS_KEY][OPEN_CCV_KEY] =
                serde_json::json!(default_settings.all_shortcuts.open_ccv);
        }
    }

    match serde_json::from_value(value) {
        Ok(settings) => settings,
        Err(_) => default_settings,
    }
}

fn get_default_settings() -> Settings {
    #[cfg(target_os = "macos")]
    let use_control = false;

    #[cfg(not(target_os = "macos"))]
    let use_control = true;

    Settings {
        notifications: Some(vec![settings::WELCOME_NOTIFICATION.to_string()]),
        version: DEFAULT_VERSION.to_string(),
        theme: Theme::Light,
        all_shortcuts: AllShortcuts {
            open_ccv: Shortcut {
                alt_key: false,
                ctrl_key: use_control,
                meta_key: !use_control,
                shift_key: false,
                code: Some("Backquote".to_string()),
            },
        },
        autostart: true,
    }
}

pub fn write_settings(app_data_dir: &PathBuf, settings: &Settings) -> Result<(), AppError> {
    let mut settings = settings.clone();
    if let Some(notifiactions) = &settings.notifications {
        if notifiactions.is_empty() {
            settings.notifications = None;
        }
    }

    let json = serde_json::to_string_pretty(&settings)
        .map_err(|err| app_error!("Unable to serialize settings. {err}"))?;

    let mut file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(app_data_dir.join(SETTINGS_FILE_NAME))
        .map_err(|err| app_error!("Unable to open settings file. {err}"))?;

    file.write_all(json.as_bytes())
        .map_err(|err| app_error!("Unable to write settings file content. {err}"))
}

pub fn store_and_notify_settings(
    app_handle: &AppHandle,
    old_settings: &Option<Settings>,
    new_settings: &Settings,
) -> Result<(), AppError> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|err| app_error!("Unable to get appication directory. {err}"))?;
    settings::core::write_settings(&app_data_dir, &new_settings)?;

    match old_settings {
        Some(old_settings) if old_settings.all_shortcuts == new_settings.all_shortcuts => {
            // nothing should be done in this case
        }
        _ => {
            settings::shortcut::register_shortcuts(app_handle, &new_settings.all_shortcuts, true)?;
        }
    }

    match old_settings {
        Some(old_settings) if old_settings.autostart == new_settings.autostart => {
            // nothing should be done in this case
        }
        _ => {
            settings::autostart::adjust_autostart(app_handle, new_settings.autostart)?;
        }
    }

    app_handle
        .emit(
            settings::SETTINGS_UPDATED_EVENT,
            EventPayload {
                data: new_settings.clone(),
            },
        )
        .map_err(|err| app_error!("{err}"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_json() {
        let expected = get_default_settings();

        let json_value = serde_json::json!("");
        let actual = fix_settings(json_value, get_default_settings());

        assert_eq!(actual, expected);
    }

    #[test]
    fn empty_notifications_for_actual_version() {
        let expected = Settings {
            notifications: None,
            version: DEFAULT_VERSION.to_string(),
            theme: Theme::Light,
            all_shortcuts: AllShortcuts {
                open_ccv: Shortcut {
                    alt_key: true,
                    ctrl_key: true,
                    meta_key: true,
                    shift_key: true,
                    code: Some("Q".to_string()),
                },
            },
            autostart: true,
        };

        let json_value = serde_json::json!({
            VERSION_KEY: DEFAULT_VERSION.to_string(),
            THEME_KEY: Theme::Light,
            AUTOSTART_KEY: true,
            ALL_SHORTCUTS_KEY: AllShortcuts{
                open_ccv: Shortcut{
                    alt_key: true,
                    shift_key: true,
                    ctrl_key: true,
                    meta_key: true,
                    code: Some("Q".to_string())
                }
            }
        });
        let actual = fix_settings(json_value, get_default_settings());

        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_notifications_for_actual_version() {
        let expected = Settings {
            notifications: None,
            version: DEFAULT_VERSION.to_string(),
            theme: Theme::Dark,
            all_shortcuts: AllShortcuts {
                open_ccv: Shortcut {
                    alt_key: true,
                    ctrl_key: true,
                    meta_key: true,
                    shift_key: true,
                    code: Some("Q".to_string()),
                },
            },
            autostart: false,
        };

        let json_value = serde_json::json!({
            VERSION_KEY: DEFAULT_VERSION.to_string(),
            NOTIFICATIONS_KEY: 123,
            THEME_KEY: Theme::Dark,
            AUTOSTART_KEY: false,
            ALL_SHORTCUTS_KEY: AllShortcuts{
                open_ccv: Shortcut{
                    alt_key: true,
                    shift_key: true,
                    ctrl_key: true,
                    meta_key: true,
                    code: Some("Q".to_string())
                }
            },
        });
        let actual = fix_settings(json_value, get_default_settings());

        assert_eq!(actual, expected);
    }

    #[test]
    fn non_empty_notifications_for_actual_version_saved() {
        let expected = Settings {
            notifications: Some(vec![settings::WELCOME_NOTIFICATION.to_string()]),
            version: DEFAULT_VERSION.to_string(),
            theme: Theme::Dark,
            all_shortcuts: AllShortcuts {
                open_ccv: Shortcut {
                    alt_key: true,
                    ctrl_key: true,
                    meta_key: true,
                    shift_key: true,
                    code: Some("Q".to_string()),
                },
            },
            autostart: true,
        };

        let json_value = serde_json::json!({
            VERSION_KEY: DEFAULT_VERSION.to_string(),
            NOTIFICATIONS_KEY: vec![settings::WELCOME_NOTIFICATION.to_string()],
            THEME_KEY: Theme::Dark,
            AUTOSTART_KEY: true,
            ALL_SHORTCUTS_KEY: AllShortcuts{
                open_ccv: Shortcut{
                    alt_key: true,
                    shift_key: true,
                    ctrl_key: true,
                    meta_key: true,
                    code: Some("Q".to_string())
                }
            }
        });
        let actual = fix_settings(json_value, get_default_settings());

        assert_eq!(actual, expected);
    }

    #[test]
    fn occupied_shortcut_notification_is_erased() {
        let expected = Settings {
            notifications: Some(vec![settings::WELCOME_NOTIFICATION.to_string()]),
            version: DEFAULT_VERSION.to_string(),
            theme: Theme::Dark,
            all_shortcuts: AllShortcuts {
                open_ccv: Shortcut {
                    alt_key: true,
                    ctrl_key: true,
                    meta_key: true,
                    shift_key: true,
                    code: Some("Q".to_string()),
                },
            },
            autostart: true,
        };

        let json_value = serde_json::json!({
            VERSION_KEY: DEFAULT_VERSION.to_string(),
            NOTIFICATIONS_KEY: vec![
                settings::WELCOME_NOTIFICATION.to_string(),
                settings::OCCUPIED_SHORTCUT_NOTIFICATION.to_string()
            ],
            THEME_KEY: Theme::Dark,
            AUTOSTART_KEY: true,
            ALL_SHORTCUTS_KEY: AllShortcuts{
                open_ccv: Shortcut{
                    alt_key: true,
                    shift_key: true,
                    ctrl_key: true,
                    meta_key: true,
                    code: Some("Q".to_string())
                }
            }
        });
        let actual = fix_settings(json_value, get_default_settings());

        assert_eq!(actual, expected);
    }

    #[test]
    fn v0_version_migrated() {
        let expected = Settings {
            notifications: Some(vec![settings::WELCOME_NOTIFICATION.to_string()]),
            version: DEFAULT_VERSION.to_string(),
            theme: Theme::Dark,
            all_shortcuts: AllShortcuts {
                open_ccv: Shortcut {
                    alt_key: true,
                    ctrl_key: true,
                    meta_key: true,
                    shift_key: true,
                    code: Some("Q".to_string()),
                },
            },
            autostart: false,
        };

        let json_value = serde_json::json!({
            VERSION_KEY: "v0",
            THEME_KEY: Theme::Dark,
            AUTOSTART_KEY: false,
            ALL_SHORTCUTS_KEY: AllShortcuts{
                open_ccv: Shortcut{
                    alt_key: true,
                    shift_key: true,
                    ctrl_key: true,
                    meta_key: true,
                    code: Some("Q".to_string())
                }
            }
        });
        let actual = fix_settings(json_value, get_default_settings());

        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_theme() {
        let expected = Settings {
            notifications: None,
            version: DEFAULT_VERSION.to_string(),
            theme: Theme::Light,
            autostart: true,
            all_shortcuts: AllShortcuts {
                open_ccv: Shortcut {
                    alt_key: true,
                    ctrl_key: true,
                    meta_key: true,
                    shift_key: true,
                    code: Some("Q".to_string()),
                },
            },
        };

        let json_value = serde_json::json!({
            VERSION_KEY: DEFAULT_VERSION.to_string(),
            THEME_KEY: "NewTheme",
            AUTOSTART_KEY: true,
            ALL_SHORTCUTS_KEY: AllShortcuts{
                open_ccv: Shortcut{
                    alt_key: true,
                    shift_key: true,
                    ctrl_key: true,
                    meta_key: true,
                    code: Some("Q".to_string())
                }
            }
        });
        let actual = fix_settings(json_value, get_default_settings());

        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_autostart() {
        let expected = Settings {
            notifications: None,
            version: DEFAULT_VERSION.to_string(),
            theme: Theme::Dark,
            autostart: true,
            all_shortcuts: AllShortcuts {
                open_ccv: Shortcut {
                    alt_key: true,
                    ctrl_key: true,
                    meta_key: true,
                    shift_key: true,
                    code: Some("Q".to_string()),
                },
            },
        };

        let json_value = serde_json::json!({
            VERSION_KEY: DEFAULT_VERSION.to_string(),
            THEME_KEY: Theme::Dark,
            AUTOSTART_KEY: "asdad",
            ALL_SHORTCUTS_KEY: AllShortcuts{
                open_ccv: Shortcut{
                    alt_key: true,
                    shift_key: true,
                    ctrl_key: true,
                    meta_key: true,
                    code: Some("Q".to_string())
                }
            }
        });
        let actual = fix_settings(json_value, get_default_settings());

        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_shortcuts() {
        let expected = Settings {
            notifications: None,
            version: DEFAULT_VERSION.to_string(),
            theme: Theme::Dark,
            autostart: false,
            all_shortcuts: get_default_settings().all_shortcuts,
        };

        let json_value = serde_json::json!({
            VERSION_KEY: DEFAULT_VERSION.to_string(),
            THEME_KEY: Theme::Dark,
            AUTOSTART_KEY: false,
            ALL_SHORTCUTS_KEY: 123
        });
        let actual = fix_settings(json_value, get_default_settings());

        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_open_ccv_shortcut() {
        let expected = Settings {
            notifications: None,
            version: DEFAULT_VERSION.to_string(),
            theme: Theme::Dark,
            autostart: false,
            all_shortcuts: get_default_settings().all_shortcuts,
        };

        let json_value = serde_json::json!({
            VERSION_KEY: DEFAULT_VERSION.to_string(),
            THEME_KEY: Theme::Dark,
            AUTOSTART_KEY: false,
            ALL_SHORTCUTS_KEY: serde_json::json!({ OPEN_CCV_KEY: 123})
        });
        let actual = fix_settings(json_value, get_default_settings());

        assert_eq!(actual, expected);
    }
}
