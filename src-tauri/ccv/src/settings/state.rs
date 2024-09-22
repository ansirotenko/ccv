use ccv_contract::models::Settings;
use std::sync::{mpsc::Sender, Mutex};

pub struct SettingsState {
    pub settings: Mutex<Option<Settings>>,
    pub settings_change: Mutex<Option<Sender<Settings>>>, // TODO maybe async Sender?
}

impl SettingsState {
    pub fn new() -> SettingsState {
        SettingsState {
            settings: Mutex::new(None),
            settings_change: Mutex::new(None),
        }
    }
}
