mod common;

use ccv_contract::{
    models::{
        CopyCategory::{Files, Html, Image, Rtf, Text, Unknown},
        CopyItem, CopyItemValue,
    },
    repository::Repository,
};
use ccv_sqlite::{
    model::{CopyItemEntity, FileInfoEntity, NewCopyItemEntity, NewFileInfoEntity},
    repository::SqliteRepository,
};
use chrono::{TimeZone, Utc};

use common::{
    copy_item_files, copy_item_html, copy_item_image, copy_item_rtf, copy_item_text,
    copy_item_unknown, get_test_data_base_url, insert_copy_items, insert_file_infos,
    load_all_copy_items, load_all_file_infos, new_copy_item_entity_files,
    new_copy_item_entity_html, new_copy_item_entity_image, new_copy_item_entity_rtf,
    new_copy_item_entity_text, new_copy_item_entity_unknown, new_file_info_entity,
};

#[test]
fn schema_pre_exists() {
    let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let database_url = get_test_data_base_url("schema_pre_exists");
    let repo = SqliteRepository::new(&database_url).unwrap();

    // migrations should have no effect
    repo.apply_migration().unwrap();

    insert_copy_items(
        &database_url,
        vec![
            new_copy_item_entity_rtf("ab", t, t),
            new_copy_item_entity_html("ab", t, t),
            new_copy_item_entity_image("ab", t, t),
            new_copy_item_entity_files("ab\nab", t, t),
            new_copy_item_entity_text("ab", t, t),
            new_copy_item_entity_unknown(t, t),
        ],
    );
    insert_file_infos(
        &database_url,
        vec![
            new_file_info_entity(4, "ab", true),
            new_file_info_entity(4, "ab", false),
        ],
    );

    let search_result = repo.search(None, 0, 1, vec![Files]).unwrap();
    let insert_result = repo.insert(&copy_item_text("", "c", t, t).value).unwrap();

    assert_eq!(search_result.items, vec![copy_item_files("4", "ab", t, t),]);
    assert_eq!(search_result.total_number, 1);
    assert_eq!(insert_result.id, "7".to_string());
}

#[test]
fn search_big_data_set() {
    let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let database_url = get_test_data_base_url("search_big_data_set");
    let repo = SqliteRepository::new(&database_url).unwrap();
    let iterations: usize = 300;
    let items_in_teration: usize = 6;

    let mut new_copy_item_entities = Vec::<NewCopyItemEntity>::new();
    let mut new_file_info_entities = Vec::<NewFileInfoEntity>::new();
    let long_text = "long long long long long long long long long long long long long long long long long long long long long long long long long long long long long long long long long long long long long long long long long long long long long long long long text";
    for i in 0..iterations {
        new_copy_item_entities.push(new_copy_item_entity_rtf(long_text, t, t));
        new_copy_item_entities.push(new_copy_item_entity_html(long_text, t, t));
        new_copy_item_entities.push(new_copy_item_entity_image(long_text, t, t));
        new_copy_item_entities.push(new_copy_item_entity_files(long_text, t, t));
        new_copy_item_entities.push(new_copy_item_entity_text(long_text, t, t));
        new_copy_item_entities.push(new_copy_item_entity_unknown(t, t));
        new_file_info_entities.push(new_file_info_entity(
            4 + (i as i32) * (items_in_teration as i32),
            long_text,
            true,
        ));
        new_file_info_entities.push(new_file_info_entity(
            4 + (i as i32) * (items_in_teration as i32),
            long_text,
            false,
        ));
    }

    insert_copy_items(&database_url, new_copy_item_entities);
    insert_file_infos(&database_url, new_file_info_entities);

    let search_result = repo
        .search(
            None,
            0,
            100000,
            vec![Unknown, Rtf, Html, Text, Files, Image],
        )
        .unwrap();

    assert_eq!(
        search_result
            .items
            .get(4 + 123 * items_in_teration - 1)
            .unwrap()
            .clone()
            .value
            .files
            .unwrap()
            .len(),
        2
    );

    assert_eq!(search_result.items.len(), items_in_teration * iterations);
    assert_eq!(search_result.total_number, items_in_teration * iterations);
}

#[test]
fn search_paging() {
    let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let t1 = Utc.with_ymd_and_hms(2001, 1, 1, 0, 0, 0).unwrap();
    let t2 = Utc.with_ymd_and_hms(2002, 1, 1, 0, 0, 0).unwrap();
    let t3 = Utc.with_ymd_and_hms(2003, 1, 1, 0, 0, 0).unwrap();
    let t4 = Utc.with_ymd_and_hms(2004, 1, 1, 0, 0, 0).unwrap();
    let t5 = Utc.with_ymd_and_hms(2005, 1, 1, 0, 0, 0).unwrap();
    let t6 = Utc.with_ymd_and_hms(2006, 1, 1, 0, 0, 0).unwrap();
    let database_url = get_test_data_base_url("search_filter_by_category");
    let repo = SqliteRepository::new(&database_url).unwrap();

    insert_copy_items(
        &database_url,
        vec![
            new_copy_item_entity_rtf("ab", t, t1),
            new_copy_item_entity_html("ab", t, t2),
            new_copy_item_entity_image("ab", t, t3),
            new_copy_item_entity_files("ab\nab", t, t4),
            new_copy_item_entity_text("ab", t, t5),
            new_copy_item_entity_unknown(t, t6),
        ],
    );
    insert_file_infos(
        &database_url,
        vec![
            new_file_info_entity(4, "ab", true),
            new_file_info_entity(4, "ab", false),
        ],
    );

    let result = repo
        .search(None, 1, 2, vec![Unknown, Image, Rtf, Files, Text, Html])
        .unwrap();

    assert_eq!(
        result.items,
        vec![
            copy_item_files("4", "ab", t, t4),
            copy_item_image("3", "ab", t, t3),
        ]
    );
    assert_eq!(result.total_number, 6);
}

#[test]
fn search_sorting() {
    let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let t1 = Utc.with_ymd_and_hms(2001, 1, 1, 0, 0, 0).unwrap();
    let t2 = Utc.with_ymd_and_hms(2002, 1, 1, 0, 0, 0).unwrap();
    let t3 = Utc.with_ymd_and_hms(2003, 1, 1, 0, 0, 0).unwrap();
    let t4 = Utc.with_ymd_and_hms(2004, 1, 1, 0, 0, 0).unwrap();
    let t5 = Utc.with_ymd_and_hms(2005, 1, 1, 0, 0, 0).unwrap();
    let t6 = Utc.with_ymd_and_hms(2006, 1, 1, 0, 0, 0).unwrap();
    let database_url = get_test_data_base_url("search_filter_by_category");
    let repo = SqliteRepository::new(&database_url).unwrap();

    insert_copy_items(
        &database_url,
        vec![
            new_copy_item_entity_rtf("ab", t, t1),
            new_copy_item_entity_html("ab", t, t2),
            new_copy_item_entity_image("ab", t, t3),
            new_copy_item_entity_files("ab\nab", t, t4),
            new_copy_item_entity_text("ab", t, t5),
            new_copy_item_entity_unknown(t, t6),
        ],
    );
    insert_file_infos(
        &database_url,
        vec![
            new_file_info_entity(4, "ab", true),
            new_file_info_entity(4, "ab", false),
        ],
    );

    let result = repo
        .search(None, 0, 100, vec![Unknown, Image, Rtf, Files, Text, Html])
        .unwrap();

    assert_eq!(
        result.items,
        vec![
            copy_item_unknown("6", t, t6),
            copy_item_text("5", "ab", t, t5),
            copy_item_files("4", "ab", t, t4),
            copy_item_image("3", "ab", t, t3),
            copy_item_html("2", "ab", t, t2),
            copy_item_rtf("1", "ab", t, t1)
        ]
    );
    assert_eq!(result.total_number, 6);
}

#[test]
fn search_filter_by_query_and_category() {
    let database_url = get_test_data_base_url("search_filter_by_category");
    let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let repo = SqliteRepository::new(&database_url).unwrap();

    insert_copy_items(
        &database_url,
        vec![
            new_copy_item_entity_rtf("ab", t, t),
            new_copy_item_entity_html("ab", t, t),
            new_copy_item_entity_image("ab", t, t),
            new_copy_item_entity_files("ab\nab", t, t),
            new_copy_item_entity_text("ab", t, t),
            new_copy_item_entity_unknown(t, t),
        ],
    );
    insert_file_infos(
        &database_url,
        vec![
            new_file_info_entity(4, "ab", true),
            new_file_info_entity(4, "ab", false),
        ],
    );

    let result = repo
        .search(Some("a"), 0, 100, vec![Unknown, Image, Rtf, Files])
        .unwrap();

    assert_eq!(
        result.items,
        vec![
            copy_item_rtf("1", "ab", t, t),
            copy_item_files("4", "ab", t, t),
        ]
    );
    assert_eq!(result.total_number, 2);
}

#[test]
fn search_filter_by_category() {
    let database_url = get_test_data_base_url("search_filter_by_category");
    let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let repo = SqliteRepository::new(&database_url).unwrap();

    insert_copy_items(
        &database_url,
        vec![
            new_copy_item_entity_rtf("ab", t, t),
            new_copy_item_entity_html("ab", t, t),
            new_copy_item_entity_image("ab", t, t),
            new_copy_item_entity_files("ab\nab", t, t),
            new_copy_item_entity_text("ab", t, t),
            new_copy_item_entity_unknown(t, t),
        ],
    );
    insert_file_infos(
        &database_url,
        vec![
            new_file_info_entity(4, "ab", true),
            new_file_info_entity(4, "ab", false),
        ],
    );

    let result = repo.search(None, 0, 100, vec![Image]).unwrap();

    assert_eq!(result.items, vec![copy_item_image("3", "ab", t, t)]);
    assert_eq!(result.total_number, 1);
}

#[test]
fn search_filter_by_query_upper_case() {
    let database_url = get_test_data_base_url("search_filter_by_query_upper_case");
    let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let repo = SqliteRepository::new(&database_url).unwrap();

    insert_copy_items(
        &database_url,
        vec![
            new_copy_item_entity_rtf("ab", t, t),
            new_copy_item_entity_html("ab", t, t),
            new_copy_item_entity_image("ab", t, t),
            new_copy_item_entity_files("ab\nab", t, t),
            new_copy_item_entity_text("ab", t, t),
            new_copy_item_entity_unknown(t, t),
        ],
    );
    insert_file_infos(
        &database_url,
        vec![
            new_file_info_entity(4, "ab", true),
            new_file_info_entity(4, "ab", false),
        ],
    );

    let result = repo
        .search(
            Some("B"),
            0,
            100,
            vec![Unknown, Html, Rtf, Image, Text, Files],
        )
        .unwrap();

    assert_eq!(
        result.items,
        vec![
            copy_item_rtf("1", "ab", t, t),
            copy_item_html("2", "ab", t, t),
            copy_item_files("4", "ab", t, t),
            copy_item_text("5", "ab", t, t)
        ]
    );
    assert_eq!(result.total_number, 4);
}

#[test]
fn search_filter_by_query_lower_case() {
    let database_url = get_test_data_base_url("search_filter_by_query_lower_case");
    let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let repo = SqliteRepository::new(&database_url).unwrap();

    insert_copy_items(
        &database_url,
        vec![
            new_copy_item_entity_rtf("AB", t, t),
            new_copy_item_entity_html("AB", t, t),
            new_copy_item_entity_image("AB", t, t),
            new_copy_item_entity_files("AB\nAB", t, t),
            new_copy_item_entity_text("AB", t, t),
            new_copy_item_entity_unknown(t, t),
        ],
    );
    insert_file_infos(
        &database_url,
        vec![
            new_file_info_entity(4, "AB", true),
            new_file_info_entity(4, "AB", false),
        ],
    );

    let result = repo
        .search(
            Some("b"),
            0,
            100,
            vec![Unknown, Html, Rtf, Image, Text, Files],
        )
        .unwrap();

    assert_eq!(
        result.items,
        vec![
            copy_item_rtf("1", "AB", t, t),
            copy_item_html("2", "AB", t, t),
            copy_item_files("4", "AB", t, t),
            copy_item_text("5", "AB", t, t)
        ]
    );
    assert_eq!(result.total_number, 4);
}

#[test]
fn search_empty_filter() {
    let database_url = get_test_data_base_url("search_empty_filter");
    let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let repo = SqliteRepository::new(&database_url).unwrap();

    insert_copy_items(
        &database_url,
        vec![
            new_copy_item_entity_rtf("ab", t, t),
            new_copy_item_entity_html("ab", t, t),
            new_copy_item_entity_image("ab", t, t),
            new_copy_item_entity_files("ab\nab", t, t),
            new_copy_item_entity_text("ab", t, t),
            new_copy_item_entity_unknown(t, t),
        ],
    );
    insert_file_infos(
        &database_url,
        vec![
            new_file_info_entity(4, "ab", true),
            new_file_info_entity(4, "ab", false),
        ],
    );

    let result = repo
        .search(None, 0, 100, vec![Unknown, Html, Rtf, Image, Text, Files])
        .unwrap();

    assert_eq!(
        result.items,
        vec![
            copy_item_rtf("1", "ab", t, t),
            copy_item_html("2", "ab", t, t),
            copy_item_image("3", "ab", t, t),
            copy_item_files("4", "ab", t, t),
            copy_item_text("5", "ab", t, t),
            copy_item_unknown("6", t, t)
        ]
    );
    assert_eq!(result.total_number, 6);
}

#[test]
fn search_filter_false_query() {
    let database_url = get_test_data_base_url("search_filter_false_query");
    let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let repo = SqliteRepository::new(&database_url).unwrap();

    insert_copy_items(
        &database_url,
        vec![
            new_copy_item_entity_rtf("ab", t, t),
            new_copy_item_entity_html("ab", t, t),
            new_copy_item_entity_image("ab", t, t),
            new_copy_item_entity_files("ab\nab", t, t),
            new_copy_item_entity_text("ab", t, t),
            new_copy_item_entity_unknown(t, t),
        ],
    );
    insert_file_infos(
        &database_url,
        vec![
            new_file_info_entity(4, "ab", true),
            new_file_info_entity(4, "ab", false),
        ],
    );

    let result = repo
        .search(
            Some("c"),
            0,
            100,
            vec![Unknown, Html, Rtf, Image, Text, Files],
        )
        .unwrap();

    assert_eq!(result.items.len(), 0);
    assert_eq!(result.total_number, 0);
}

#[test]
fn search_filter_empty_categories() {
    let database_url = get_test_data_base_url("search_filter_empty_categories");
    let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let repo = SqliteRepository::new(&database_url).unwrap();

    insert_copy_items(
        &database_url,
        vec![
            new_copy_item_entity_rtf("ab", t, t),
            new_copy_item_entity_html("ab", t, t),
            new_copy_item_entity_image("ab", t, t),
            new_copy_item_entity_files("ab\nab", t, t),
            new_copy_item_entity_text("ab", t, t),
            new_copy_item_entity_unknown(t, t),
        ],
    );
    insert_file_infos(
        &database_url,
        vec![
            new_file_info_entity(4, "ab", true),
            new_file_info_entity(4, "ab", false),
        ],
    );

    let result = repo.search(Some("a"), 0, 100, Vec::new()).unwrap();

    assert_eq!(result.items.len(), 0);
    assert_eq!(result.total_number, 0);
}

#[test]
#[should_panic]
fn search_filter_wrong_page() {
    let database_url = get_test_data_base_url("search_filter_wrong_page");
    let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let repo = SqliteRepository::new(&database_url).unwrap();

    insert_copy_items(
        &database_url,
        vec![
            new_copy_item_entity_rtf("ab", t, t),
            new_copy_item_entity_html("ab", t, t),
            new_copy_item_entity_image("ab", t, t),
            new_copy_item_entity_files("ab\nab", t, t),
            new_copy_item_entity_text("ab", t, t),
            new_copy_item_entity_unknown(t, t),
        ],
    );
    insert_file_infos(
        &database_url,
        vec![
            new_file_info_entity(4, "ab", true),
            new_file_info_entity(4, "ab", false),
        ],
    );

    repo.search(None, -1, 100, vec![Unknown, Html, Rtf, Image, Text, Files])
        .unwrap();
}

#[test]
#[should_panic]
fn search_filter_wrong_page_size() {
    let database_url = get_test_data_base_url("search_filter_wrong_page_size");
    let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let repo = SqliteRepository::new(&database_url).unwrap();

    insert_copy_items(
        &database_url,
        vec![
            new_copy_item_entity_rtf("ab", t, t),
            new_copy_item_entity_html("ab", t, t),
            new_copy_item_entity_image("ab", t, t),
            new_copy_item_entity_files("ab\nab", t, t),
            new_copy_item_entity_text("ab", t, t),
            new_copy_item_entity_unknown(t, t),
        ],
    );
    insert_file_infos(
        &database_url,
        vec![
            new_file_info_entity(4, "ab", true),
            new_file_info_entity(4, "ab", false),
        ],
    );

    repo.search(None, 0, -100, vec![Unknown, Html, Rtf, Image, Text, Files])
        .unwrap();
}

#[test]
fn insert_unknown_happy_path() {
    let t = Utc::now();
    let new_item = copy_item_unknown("", t, t);
    insert_item_happy_path(
        "insert_unknown_happy_path",
        &new_item.value,
        &new_item,
        &CopyItemEntity {
            category: "Unknown".to_string(),
            id: -1,
            created: t.naive_utc(),
            last_reuse: t.naive_utc(),
            files_text: None,
            html: None,
            text: None,
            image: None,
            rtf: None,
        },
        &Vec::new(),
    );
}

#[test]
fn insert_image_happy_path() {
    let t = Utc::now();
    let new_item = copy_item_image("", "a", t, t);
    insert_item_happy_path(
        "insert_image_happy_path",
        &new_item.value,
        &new_item,
        &CopyItemEntity {
            category: "Image".to_string(),
            id: -1,
            created: t.naive_utc(),
            last_reuse: t.naive_utc(),
            files_text: None,
            html: None,
            text: None,
            image: Some("a".to_string()),
            rtf: None,
        },
        &Vec::new(),
    );
}

#[test]
fn insert_rtf_happy_path() {
    let t = Utc::now();
    let new_item = copy_item_rtf("", "a", t, t);
    insert_item_happy_path(
        "insert_rtf_happy_path",
        &new_item.value,
        &new_item,
        &CopyItemEntity {
            category: "Rtf".to_string(),
            id: -1,
            created: t.naive_utc(),
            last_reuse: t.naive_utc(),
            files_text: None,
            html: None,
            text: Some("a".to_string()),
            image: None,
            rtf: Some("a".to_string()),
        },
        &Vec::new(),
    );
}

#[test]
fn insert_html_happy_path() {
    let t = Utc::now();
    let new_item = copy_item_html("", "a", t, t);
    insert_item_happy_path(
        "insert_html_happy_path",
        &new_item.value,
        &new_item,
        &CopyItemEntity {
            category: "Html".to_string(),
            id: -1,
            created: t.naive_utc(),
            last_reuse: t.naive_utc(),
            files_text: None,
            html: Some("a".to_string()),
            text: Some("a".to_string()),
            image: None,
            rtf: None,
        },
        &Vec::new(),
    );
}

#[test]
fn insert_text_happy_path() {
    let t = Utc::now();
    let new_item = copy_item_text("", "a", t, t);
    insert_item_happy_path(
        "insert_text_happy_path",
        &new_item.value,
        &new_item,
        &CopyItemEntity {
            category: "Text".to_string(),
            id: -1,
            created: t.naive_utc(),
            last_reuse: t.naive_utc(),
            files_text: None,
            html: None,
            text: Some("a".to_string()),
            image: None,
            rtf: None,
        },
        &Vec::new(),
    );
}

#[test]
fn insert_files_happy_path() {
    let t = Utc::now();
    let new_item = copy_item_files("", "a", t, t);
    insert_item_happy_path(
        "insert_files_happy_path",
        &new_item.value,
        &new_item,
        &CopyItemEntity {
            category: "Files".to_string(),
            id: -1,
            created: t.naive_utc(),
            last_reuse: t.naive_utc(),
            files_text: Some("a\na".to_string()),
            html: None,
            text: None,
            image: None,
            rtf: None,
        },
        &vec![
            FileInfoEntity {
                id: -1,
                full_path: "a".to_string(),
                directory_path: Some("a".to_string()),
                file_name: Some("a".to_string()),
                icon_base64: None,
                is_directory: true,
                copy_item_id: -1,
            },
            FileInfoEntity {
                id: -1,
                full_path: "a".to_string(),
                directory_path: Some("a".to_string()),
                file_name: Some("a".to_string()),
                icon_base64: None,
                is_directory: false,
                copy_item_id: -1,
            },
        ],
    );
}

fn insert_item_happy_path(
    test_name: &str,
    new_copy_item_value: &CopyItemValue,
    expected_copy_item: &CopyItem,
    expected_copy_item_entity: &CopyItemEntity,
    expected_file_info_entities: &[FileInfoEntity],
) {
    let database_url = get_test_data_base_url(test_name);
    let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let repo = SqliteRepository::new(&database_url).unwrap();

    let inserted_copy_items = insert_copy_items(
        &database_url,
        vec![
            new_copy_item_entity_unknown(t, t),
            new_copy_item_entity_text("aa1", t, t),
            new_copy_item_entity_rtf("aa1", t, t),
            new_copy_item_entity_html("aa1", t, t),
            new_copy_item_entity_image("aa1", t, t),
            new_copy_item_entity_files("aa1\naa1", t, t),
        ],
    );

    let expected_count: usize = inserted_copy_items.len() + 1;

    let inserted_file_infos = insert_file_infos(
        &database_url,
        vec![
            new_file_info_entity(6, "aa1", true),
            new_file_info_entity(6, "aa1", false),
        ],
    );

    let result = repo.insert(new_copy_item_value).unwrap();

    let loaded_copy_items = load_all_copy_items(&database_url);
    let create_time = loaded_copy_items
        .get(expected_count - 1)
        .unwrap()
        .created
        .and_utc();

    let mut expected_fixed_copy_item_entity = expected_copy_item_entity.clone();
    expected_fixed_copy_item_entity.id = expected_count as i32;
    expected_fixed_copy_item_entity.created = create_time.naive_utc();
    expected_fixed_copy_item_entity.last_reuse = create_time.naive_utc();
    assert_eq!(
        &loaded_copy_items[..],
        [&inserted_copy_items[..], &[expected_fixed_copy_item_entity]].concat()
    );

    let mut expected_fixed_copy_item = expected_copy_item.clone();
    expected_fixed_copy_item.created = create_time;
    expected_fixed_copy_item.last_reuse = create_time;
    expected_fixed_copy_item.id = expected_count.to_string();
    assert_eq!(result, expected_fixed_copy_item);

    let mut inserted_file_infos_len: i32 = inserted_file_infos.len() as i32;
    let loaded_file_infos = load_all_file_infos(&database_url);
    let expected_fixed_file_info_entities: Vec<FileInfoEntity> = expected_file_info_entities
        .iter()
        .map(|f| {
            let mut ret = f.clone();
            inserted_file_infos_len += 1;
            ret.id = inserted_file_infos_len;
            ret.copy_item_id = expected_count as i32;
            return ret;
        })
        .collect();
    assert_eq!(
        &loaded_file_infos[..],
        [
            &inserted_file_infos[..],
            &expected_fixed_file_info_entities[..]
        ]
        .concat()
    );
}

#[test]
fn update_last_resue_happy_path() {
    let database_url = get_test_data_base_url("update_last_resue_happy_path");
    let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let repo = SqliteRepository::new(&database_url).unwrap();

    let inserted_copy_items = insert_copy_items(
        &database_url,
        vec![
            new_copy_item_entity_rtf("aa1", t, t),
            new_copy_item_entity_html("aa1", t, t),
        ],
    );

    let new_last_reuse = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
    let result = repo.update_last_resue("1", new_last_reuse).unwrap();

    let loaded_copy_items = load_all_copy_items(&database_url);
    assert_eq!(loaded_copy_items.len(), 2);
    assert_eq!(
        loaded_copy_items.get(0).unwrap().id,
        inserted_copy_items.get(0).unwrap().id
    );
    assert_eq!(
        loaded_copy_items.get(0).unwrap().created,
        inserted_copy_items.get(0).unwrap().created
    );
    assert_eq!(
        loaded_copy_items.get(0).unwrap().last_reuse,
        new_last_reuse.naive_utc()
    );
    assert_eq!(
        loaded_copy_items.get(1).unwrap(),
        inserted_copy_items.get(1).unwrap()
    );
    assert_eq!(result, copy_item_rtf("1", "aa1", t, new_last_reuse))
}

#[test]
#[should_panic]
fn update_last_resue_has_no_effect() {
    let database_url = get_test_data_base_url("update_last_resue_has_no_effect");
    let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let repo = SqliteRepository::new(&database_url).unwrap();

    insert_copy_items(
        &database_url,
        vec![
            new_copy_item_entity_rtf("aa1", t, t),
            new_copy_item_entity_html("aa1", t, t),
        ],
    );

    let last_reuse = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
    repo.update_last_resue("3", last_reuse).unwrap();
}

#[test]
fn find_by_value_happy_path() {
    let database_url = get_test_data_base_url("find_by_value_happy_path");
    let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let repo = SqliteRepository::new(&database_url).unwrap();

    let rtf = copy_item_rtf("1", "aa1", t, t);
    let html = copy_item_html("2", "aa1", t, t);
    let image = copy_item_image("3", "aa1", t, t);
    let files = copy_item_files("4", "aa1", t, t);
    let text = copy_item_text("5", "aa1", t, t);
    let unknown = copy_item_unknown("6", t, t);

    insert_copy_items(
        &database_url,
        vec![
            new_copy_item_entity_rtf("aa1", t, t),
            new_copy_item_entity_html("aa1", t, t),
            new_copy_item_entity_image("aa1", t, t),
            new_copy_item_entity_files("aa1\naa1", t, t),
            new_copy_item_entity_text("aa1", t, t),
            new_copy_item_entity_unknown(t, t),
        ],
    );

    insert_file_infos(
        &database_url,
        vec![
            new_file_info_entity(4, "aa1", true),
            new_file_info_entity(4, "aa1", false),
        ],
    );

    assert_eq!(repo.find_by_value(&rtf.value).unwrap().unwrap(), rtf);
    assert_eq!(repo.find_by_value(&html.value).unwrap().unwrap(), html);
    assert_eq!(repo.find_by_value(&image.value).unwrap().unwrap(), image);
    assert_eq!(repo.find_by_value(&files.value).unwrap().unwrap(), files);
    assert_eq!(repo.find_by_value(&text.value).unwrap().unwrap(), text);
    assert_eq!(
        repo.find_by_value(&unknown.value).unwrap().unwrap(),
        unknown
    );
    assert!(repo
        .find_by_value(&copy_item_rtf("1", "b", t, t).value)
        .unwrap()
        .is_none());
    assert!(repo
        .find_by_value(&copy_item_html("2", "b", t, t).value)
        .unwrap()
        .is_none());
    assert!(repo
        .find_by_value(&copy_item_image("3", "b", t, t).value)
        .unwrap()
        .is_none());
    assert!(repo
        .find_by_value(&copy_item_files("4", "b", t, t).value)
        .unwrap()
        .is_none());
    assert!(repo
        .find_by_value(&copy_item_text("5", "b", t, t).value)
        .unwrap()
        .is_none());
}

#[test]
fn remove_happy_path() {
    let database_url = get_test_data_base_url("remove_happy_path");
    let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let repo = SqliteRepository::new(&database_url).unwrap();

    let copy_item1 = new_copy_item_entity_text("sss1", t, t);
    let copy_item2 = new_copy_item_entity_files("sss2", t, t);
    let copy_item3 = new_copy_item_entity_files("sss3_1\nsss3_2", t, t);
    let inserted_copy_items =
        insert_copy_items(&database_url, vec![copy_item1, copy_item2, copy_item3]);

    let file_item2 = new_file_info_entity(2, "sss2", true);
    let file_item3_1 = new_file_info_entity(3, "sss3_1", true);
    let file_item3_2 = new_file_info_entity(3, "sss3_2", false);
    let inserted_file_infos =
        insert_file_infos(&database_url, vec![file_item2, file_item3_1, file_item3_2]);

    repo.remove(&vec!["1", "3"]).unwrap();

    let loaded_copy_items = load_all_copy_items(&database_url);
    assert_eq!(&loaded_copy_items[..], &inserted_copy_items[1..2]);

    let loaded_file_infos = load_all_file_infos(&database_url);
    assert_eq!(&loaded_file_infos[..], &inserted_file_infos[..1]);
}

#[test]
fn remove_has_no_effect() {
    let database_url = get_test_data_base_url("remove_has_no_effect");
    let t = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
    let repo = SqliteRepository::new(&database_url).unwrap();

    let copy_item1 = new_copy_item_entity_text("sss1", t, t);
    let copy_item2 = new_copy_item_entity_files("sss2", t, t);
    let inserted_copy_items = insert_copy_items(&database_url, vec![copy_item1, copy_item2]);

    let file_item2 = new_file_info_entity(2, "sss2", true);
    let inserted_file_infos = insert_file_infos(&database_url, vec![file_item2]);

    repo.remove(&vec!["4", "3"]).unwrap();

    let loaded_copy_items = load_all_copy_items(&database_url);
    assert_eq!(loaded_copy_items, inserted_copy_items);

    let loaded_file_infos = load_all_file_infos(&database_url);
    assert_eq!(loaded_file_infos, inserted_file_infos);
}

#[test]
fn remove_older_happy_path() {
    let database_url = get_test_data_base_url("remove_older_happy_path");
    let t = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let t1 = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let t2 = Utc.with_ymd_and_hms(2001, 1, 1, 0, 0, 0).unwrap();
    let t3 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let repo = SqliteRepository::new(&database_url).unwrap();
    let sinse = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();

    let t1_copy_item = new_copy_item_entity_text("sss1", t, t1);
    let t2_copy_item = new_copy_item_entity_files("sss2_1\nsss2_2", t, t2);
    let t3_copy_item = new_copy_item_entity_files("sss3", t, t3);
    let inserted_copy_items = insert_copy_items(
        &database_url,
        vec![t1_copy_item, t2_copy_item, t3_copy_item],
    );

    let t2_1_file_item = new_file_info_entity(2, "sss2_1", true);
    let t2_2_file_item = new_file_info_entity(2, "sss2_2", false);
    let t3_file_item = new_file_info_entity(3, "sss3", true);
    let inserted_file_infos = insert_file_infos(
        &database_url,
        vec![t2_1_file_item, t2_2_file_item, t3_file_item],
    );

    repo.remove_older(sinse).unwrap();

    let loaded_copy_items = load_all_copy_items(&database_url);
    assert_eq!(&loaded_copy_items[..], &inserted_copy_items[2..]);

    let loaded_file_infos = load_all_file_infos(&database_url);
    assert_eq!(&loaded_file_infos[..], &inserted_file_infos[2..]);
}

#[test]
fn remove_older_has_no_effect() {
    let database_url = get_test_data_base_url("remove_older_has_no_effect");
    let t = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
    let t1 = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
    let t2 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let repo = SqliteRepository::new(&database_url).unwrap();
    let sinse = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();

    let t1_copy_item = new_copy_item_entity_text("sss1", t, t1);
    let t2_copy_item = new_copy_item_entity_files("sss2", t, t2);
    let inserted_copy_items = insert_copy_items(&database_url, vec![t1_copy_item, t2_copy_item]);

    let t2_file_item = new_file_info_entity(2, "sss2", true);
    let inserted_file_infos = insert_file_infos(&database_url, vec![t2_file_item]);

    repo.remove_older(sinse).unwrap();

    let loaded_copy_items = load_all_copy_items(&database_url);
    assert_eq!(loaded_copy_items, inserted_copy_items);

    let loaded_file_infos = load_all_file_infos(&database_url);
    assert_eq!(loaded_file_infos, inserted_file_infos);
}
