use crate::schema::{copy_items, file_infos};
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Clone)]
#[diesel(table_name = copy_items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CopyItemEntity {
    pub id: i32,
    pub created: NaiveDateTime,
    pub last_reuse: NaiveDateTime,
    pub category: String,
    pub text: Option<String>,
    pub html: Option<String>,
    pub rtf: Option<String>,
    pub files_text: Option<String>,
    pub image: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = copy_items)]
pub struct NewCopyItemEntity<'a> {
    pub created: NaiveDateTime,
    pub last_reuse: NaiveDateTime,
    pub category: &'a str,
    pub text: Option<&'a str>,
    pub html: Option<&'a str>,
    pub rtf: Option<&'a str>,
    pub files_text: Option<&'a str>,
    pub image: Option<&'a str>,
}

#[derive(Queryable, Selectable, PartialEq, Identifiable, Associations, Debug, Clone)]
#[diesel(table_name = file_infos)]
#[diesel(belongs_to(CopyItemEntity, foreign_key = copy_item_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct FileInfoEntity {
    pub id: i32,
    pub copy_item_id: i32,
    pub full_path: String,
    pub file_name: Option<String>,
    pub directory_path: Option<String>,
    pub icon_base64: Option<String>,
    pub is_directory: bool,
}

#[derive(Insertable)]
#[diesel(table_name = file_infos)]
pub struct NewFileInfoEntity<'a> {
    pub copy_item_id: i32,
    pub full_path: &'a str,
    pub file_name: Option<&'a str>,
    pub directory_path: Option<&'a str>,
    pub icon_base64: Option<&'a str>,
    pub is_directory: bool,
}
