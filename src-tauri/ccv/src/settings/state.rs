use ccv_contract::models::Settings;
use std::sync::{mpsc::Sender, Mutex};

pub struct SettingsState {
    pub settings: Mutex<Option<Settings>>,
    pub hotkey_change: Mutex<Option<Sender<Settings>>>,
}

impl SettingsState {
    pub fn new() -> SettingsState {
        SettingsState {
            settings: Mutex::new(None),
            hotkey_change: Mutex::new(None),
        }
    }
}
