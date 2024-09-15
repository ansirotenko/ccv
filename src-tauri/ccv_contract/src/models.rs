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

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct AboutData {
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "authors")]
    pub authors: String,
    #[serde(rename = "homepage")]
    pub homepage: String,
    #[serde(rename = "appDirectory")]
    pub app_dir: Option<String>,
    #[serde(rename = "appDataDirectory")]
    pub app_data_dir: Option<String>,
    #[serde(rename = "appLogsDirectory")]
    pub app_logs_dir: Option<String>,
    #[serde(rename = "text")]
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Keybindings {
    #[serde(rename = "openCcv")]
    pub open_ccv: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Settings {
    #[serde(rename = "keybindings")]
    pub keybindings: Keybindings,
    #[serde(rename = "theme")]
    pub theme: Theme,
}

#[derive(Clone, Serialize, Debug)]
pub struct EventPayload<T> {
    #[serde(rename = "data")]
    pub data: T,
}
