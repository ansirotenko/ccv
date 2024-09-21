use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    sync::Mutex,
};

use ccv_contract::{
    app_error,
    error::AppError,
    models::{Keybindings, Settings, Shortcut, Theme::Light},
    repository::Repository,
};

pub struct CopyItemState {
    pub repository: Mutex<Box<dyn Repository + Send + Sync + 'static>>,
}

impl CopyItemState {
    pub fn new_uninitialized() -> CopyItemState {
        CopyItemState {
            repository: Mutex::new(Box::new(UninitalizedRepository {})),
        }
    }
}

struct UninitalizedRepository;

impl Repository for UninitalizedRepository {
    fn search(
        &self,
        _query: Option<&str>,
        _page: i32,
        _page_size: i32,
        _categories: Vec<ccv_contract::models::CopyCategory>,
    ) -> Result<ccv_contract::models::SearchResult, AppError> {
        Err(app_error!("Repository is unitialzed"))
    }

    fn insert(
        &self,
        _item_value: &ccv_contract::models::CopyItemValue,
    ) -> Result<ccv_contract::models::CopyItem, AppError> {
        Err(app_error!("Repository is unitialzed"))
    }

    fn find_by_value(
        &self,
        _item_value: &ccv_contract::models::CopyItemValue,
    ) -> Result<Option<ccv_contract::models::CopyItem>, AppError> {
        Err(app_error!("Repository is unitialzed"))
    }

    fn update_last_resue(
        &self,
        _item_id: &str,
        _new_last_reuse: chrono::DateTime<chrono::Utc>,
    ) -> Result<ccv_contract::models::CopyItem, AppError> {
        Err(app_error!("Repository is unitialzed"))
    }

    fn remove(&self, _item_ids: &Vec<&str>) -> Result<(), AppError> {
        Err(app_error!("Repository is unitialzed"))
    }

    fn remove_older(&self, _sinse: chrono::DateTime<chrono::Utc>) -> Result<(), AppError> {
        Err(app_error!("Repository is unitialzed"))
    }
}

pub struct SettingsState {
    pub settings: Mutex<Option<Settings>>,
}

const SETTINGS_FILE_NAME: &str = "settings.json";

impl SettingsState {
    pub fn new() -> SettingsState {
        SettingsState {
            settings: Mutex::new(None),
        }
    }

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
                theme: Light,
                keybindings: Keybindings {
                    open_ccv: Shortcut {
                        alt_key: true,
                        ctrl_key: false,
                        meta_key: false,
                        shift_key: false,
                        code: Some("KeyV".to_string()),
                    },
                },
            };

            SettingsState::write_settings(app_data_dir, &default_settings)?;
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
}
