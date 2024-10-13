use ccv_contract::models::Settings;
use std::sync::Mutex;

pub struct SettingsState {
    pub settings: Mutex<Option<Settings>>,
}

impl SettingsState {
    pub fn new() -> SettingsState {
        SettingsState {
            settings: Mutex::new(None),
        }
    }
}
