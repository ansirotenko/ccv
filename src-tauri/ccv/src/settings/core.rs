use ccv_contract::{
    app_error,
    error::AppError,
    models::{AllShortcuts, Settings, Shortcut, Theme},
};
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

const SETTINGS_FILE_NAME: &str = "settings.json";

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

    if let Ok(result) = serde_json::from_str(&file_content) {
        Ok(result)
    } else {
        let default_settings = Settings {
            theme: Theme::Light,
            all_shortcuts: AllShortcuts {
                open_ccv: Shortcut {
                    alt_key: true,
                    ctrl_key: false,
                    meta_key: false,
                    shift_key: false,
                    code: Some("KeyV".to_string()),
                },
            },
        };

        write_settings(app_data_dir, &default_settings)?;
        Ok(default_settings)
    }
}

pub fn write_settings(app_data_dir: &PathBuf, settings: &Settings) -> Result<(), AppError> {
    let json = serde_json::to_string_pretty(settings)
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
