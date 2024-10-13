use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum CopyCategory {
    Text,
    Html,
    Rtf,
    Image,
    Files,
    Unknown,
}

#[derive(Serialize, Clone, Debug)]
pub struct SearchResult {
    #[serde(rename = "items")]
    pub items: Vec<CopyItem>,
    #[serde(rename = "totalNumber")]
    pub total_number: usize,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct CopyItem {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "created")]
    pub created: DateTime<Utc>,
    #[serde(rename = "lastReuse")]
    pub last_reuse: DateTime<Utc>,
    #[serde(rename = "value")]
    pub value: CopyItemValue,
}

#[derive(Serialize, Clone, PartialEq, Debug)]
pub struct CopyItemValue {
    #[serde(rename = "category")]
    pub category: CopyCategory,
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[serde(rename = "html")]
    pub html: Option<String>,
    #[serde(rename = "rtf")]
    pub rtf: Option<String>,
    #[serde(rename = "image")]
    pub image: Option<String>,
    #[serde(rename = "files")]
    pub files: Option<Vec<FileInfo>>,
}

#[derive(Serialize, Clone, PartialEq, Debug)]
pub struct FileInfo {
    #[serde(rename = "fullPath")]
    pub full_path: String,
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
    #[serde(rename = "directoryPath")]
    pub directory_path: Option<String>,
    #[serde(rename = "iconBase64")]
    pub icon_base64: Option<String>,
    #[serde(rename = "isDirectory")]
    pub is_directory: bool,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum Os {
    Linux,
    MacOs,
    Windows
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct AboutData {
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "authors")]
    pub authors: String,
    #[serde(rename = "homepage")]
    pub homepage: String,
    #[serde(rename = "appDirectory")]
    pub app_dir: String,
    #[serde(rename = "appDataDirectory")]
    pub app_data_dir: String,
    #[serde(rename = "appLogsDirectory")]
    pub app_logs_dir: String,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "os")]
    pub os: Os,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Shortcut {
    #[serde(rename = "altKey")]
    pub alt_key: bool,
    #[serde(rename = "ctrlKey")]
    pub ctrl_key: bool,
    #[serde(rename = "shiftKey")]
    pub shift_key: bool,
    #[serde(rename = "metaKey")]
    pub meta_key: bool,
    #[serde(rename = "code")]
    pub code: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AllShortcuts {
    #[serde(rename = "openCcv")]
    pub open_ccv: Shortcut,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Settings {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "notifications")]
    pub notifications: Option<Vec<String>>,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "allShortcuts")]
    pub all_shortcuts: AllShortcuts,
    #[serde(rename = "theme")]
    pub theme: Theme,
    #[serde(rename = "autostart")]
    pub autostart: bool,
}

#[derive(Clone, Serialize, Debug)]
pub struct EventPayload<T> {
    #[serde(rename = "data")]
    pub data: T,
}

#[derive(Clone, Serialize, Debug)]
pub struct MainShortcutPressedPayload {
    #[serde(rename = "changedFromHiddenToVisile")]
    pub changed_from_hidden_to_visile: bool 
}