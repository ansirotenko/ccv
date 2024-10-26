use ccv_contract::models::{
    CopyCategory::{Files, Html, Image, Rtf, Text, Unknown},
    CopyItem, CopyItemValue, FileInfo,
};
use ccv_sqlite::{
    enum_converter::from_copy_category,
    model::{CopyItemEntity, FileInfoEntity, NewCopyItemEntity, NewFileInfoEntity},
};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::SqliteConnection;

pub fn copy_item_unknown(id: &str, created: DateTime<Utc>, last_reuse: DateTime<Utc>) -> CopyItem {
    CopyItem {
        id: id.to_string(),
        created: created,
        last_reuse: last_reuse,
        value: CopyItemValue {
            category: Unknown,
            image: None,
            rtf: None,
            files: None,
            text: None,
            html: None,
        },
    }
}
pub fn copy_item_text(
    id: &str,
    s: &str,
    created: DateTime<Utc>,
    last_reuse: DateTime<Utc>,
) -> CopyItem {
    CopyItem {
        id: id.to_string(),
        created: created,
        last_reuse: last_reuse,
        value: CopyItemValue {
            category: Text,
            image: None,
            rtf: None,
            files: None,
            text: Some(s.to_string()),
            html: None,
        },
    }
}
pub fn copy_item_image(
    id: &str,
    s: &str,
    created: DateTime<Utc>,
    last_reuse: DateTime<Utc>,
) -> CopyItem {
    CopyItem {
        id: id.to_string(),
        created: created,
        last_reuse: last_reuse,
        value: CopyItemValue {
            category: Image,
            image: Some(s.to_string()),
            rtf: None,
            files: None,
            text: None,
            html: None,
        },
    }
}
pub fn copy_item_html(
    id: &str,
    s: &str,
    created: DateTime<Utc>,
    last_reuse: DateTime<Utc>,
) -> CopyItem {
    CopyItem {
        id: id.to_string(),
        created: created,
        last_reuse: last_reuse,
        value: CopyItemValue {
            category: Html,
            image: None,
            rtf: None,
            files: None,
            text: Some(s.to_string()),
            html: Some(s.to_string()),
        },
    }
}
pub fn copy_item_rtf(
    id: &str,
    s: &str,
    created: DateTime<Utc>,
    last_reuse: DateTime<Utc>,
) -> CopyItem {
    CopyItem {
        id: id.to_string(),
        created: created,
        last_reuse: last_reuse,
        value: CopyItemValue {
            category: Rtf,
            image: None,
            rtf: Some(s.to_string()),
            files: None,
            text: Some(s.to_string()),
            html: None,
        },
    }
}
pub fn copy_item_files(
    id: &str,
    s: &str,
    created: DateTime<Utc>,
    last_reuse: DateTime<Utc>,
) -> CopyItem {
    let files = vec![
        FileInfo {
            directory_path: Some(s.to_string()),
            full_path: s.to_string(),
            file_name: Some(s.to_string()),
            icon_base64: None,
            is_directory: true,
        },
        FileInfo {
            directory_path: Some(s.to_string()),
            full_path: s.to_string(),
            file_name: Some(s.to_string()),
            icon_base64: None,
            is_directory: false,
        },
    ];
    CopyItem {
        id: id.to_string(),
        created: created,
        last_reuse: last_reuse,
        value: CopyItemValue {
            category: Files,
            image: None,
            rtf: None,
            files: Some(files),
            text: None,
            html: None,
        },
    }
}

pub fn new_copy_item_entity_unknown<'a>(
    created: DateTime<Utc>,
    last_reuse: DateTime<Utc>,
) -> NewCopyItemEntity<'a> {
    NewCopyItemEntity {
        category: from_copy_category(Unknown),
        created: created.naive_utc(),
        last_reuse: last_reuse.naive_utc(),
        files_text: None,
        text: None,
        html: None,
        image: None,
        rtf: None,
    }
}
pub fn new_copy_item_entity_text<'a>(
    s: &'a str,
    created: DateTime<Utc>,
    last_reuse: DateTime<Utc>,
) -> NewCopyItemEntity<'a> {
    NewCopyItemEntity {
        category: from_copy_category(Text),
        created: created.naive_utc(),
        last_reuse: last_reuse.naive_utc(),
        files_text: None,
        text: Some(s),
        html: None,
        image: None,
        rtf: None,
    }
}
pub fn new_copy_item_entity_image<'a>(
    s: &'a str,
    created: DateTime<Utc>,
    last_reuse: DateTime<Utc>,
) -> NewCopyItemEntity<'a> {
    NewCopyItemEntity {
        category: from_copy_category(Image),
        created: created.naive_utc(),
        last_reuse: last_reuse.naive_utc(),
        files_text: None,
        text: None,
        html: None,
        image: Some(s),
        rtf: None,
    }
}
pub fn new_copy_item_entity_html<'a>(
    s: &'a str,
    created: DateTime<Utc>,
    last_reuse: DateTime<Utc>,
) -> NewCopyItemEntity<'a> {
    NewCopyItemEntity {
        category: from_copy_category(Html),
        created: created.naive_utc(),
        last_reuse: last_reuse.naive_utc(),
        files_text: None,
        text: Some(s),
        html: Some(s),
        image: None,
        rtf: None,
    }
}
pub fn new_copy_item_entity_rtf<'a>(
    s: &'a str,
    created: DateTime<Utc>,
    last_reuse: DateTime<Utc>,
) -> NewCopyItemEntity<'a> {
    NewCopyItemEntity {
        category: from_copy_category(Rtf),
        created: created.naive_utc(),
        last_reuse: last_reuse.naive_utc(),
        files_text: None,
        text: Some(s),
        html: None,
        image: None,
        rtf: Some(s),
    }
}
pub fn new_copy_item_entity_files<'a>(
    s: &'a str,
    created: DateTime<Utc>,
    last_reuse: DateTime<Utc>,
) -> NewCopyItemEntity<'a> {
    NewCopyItemEntity {
        category: from_copy_category(Files),
        created: created.naive_utc(),
        last_reuse: last_reuse.naive_utc(),
        files_text: Some(s),
        text: None,
        html: None,
        image: None,
        rtf: None,
    }
}
pub fn new_file_info_entity<'a>(
    copy_item_id: i32,
    s: &'a str,
    is_dir: bool,
) -> NewFileInfoEntity<'a> {
    NewFileInfoEntity {
        copy_item_id: copy_item_id,
        full_path: s,
        directory_path: Some(s),
        file_name: Some(s),
        icon_base64: None,
        is_directory: is_dir,
    }
}

pub fn load_all_file_infos(database_url: &str) -> Vec<FileInfoEntity> {
    let mut connection = SqliteConnection::establish(&database_url).unwrap();

    use ccv_sqlite::schema::file_infos::dsl::file_infos;
    file_infos
        .select(FileInfoEntity::as_select())
        .get_results(&mut connection)
        .unwrap()
}

pub fn load_all_copy_items(database_url: &str) -> Vec<CopyItemEntity> {
    let mut connection = SqliteConnection::establish(&database_url).unwrap();

    use ccv_sqlite::schema::copy_items::dsl::copy_items;
    copy_items
        .select(CopyItemEntity::as_select())
        .get_results(&mut connection)
        .unwrap()
}

pub fn insert_file_infos(
    database_url: &str,
    new_file_infos: Vec<NewFileInfoEntity>,
) -> Vec<FileInfoEntity> {
    let mut connection = SqliteConnection::establish(&database_url).unwrap();

    use ccv_sqlite::schema::file_infos::dsl::file_infos;

    let mut result: Vec<FileInfoEntity> = vec![];

    for new_file_info in new_file_infos {
        let inserted: FileInfoEntity = diesel::insert_into(file_infos)
            .values(&new_file_info)
            .returning(FileInfoEntity::as_select())
            .get_result(&mut connection)
            .unwrap();

        result.push(inserted);
    }

    result
}

pub fn insert_copy_items(
    database_url: &str,
    new_copy_items: Vec<NewCopyItemEntity>,
) -> Vec<CopyItemEntity> {
    let mut connection = SqliteConnection::establish(&database_url).unwrap();

    use ccv_sqlite::schema::copy_items::dsl::copy_items;

    let mut result: Vec<CopyItemEntity> = vec![];

    for new_copy_item in new_copy_items {
        let inserted: CopyItemEntity = diesel::insert_into(copy_items)
            .values(&new_copy_item)
            .returning(CopyItemEntity::as_select())
            .get_result(&mut connection)
            .unwrap();

        result.push(inserted);
    }

    result
}

pub fn get_test_data_base_url(name: &str) -> String {
    let now = Utc::now().format("%Y-%m-%d_%H-%M-%S-%9f");
    format!("tests/db/test-{name}-{now}.db")
}
