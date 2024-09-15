// @generated automatically by Diesel CLI.

diesel::table! {
    copy_items (id) {
        id -> Integer,
        created -> Timestamp,
        last_reuse -> Timestamp,
        category -> Text,
        text -> Nullable<Text>,
        html -> Nullable<Text>,
        rtf -> Nullable<Text>,
        files_text -> Nullable<Text>,
        image -> Nullable<Text>,
    }
}

diesel::table! {
    file_infos (id) {
        id -> Integer,
        copy_item_id -> Integer,
        full_path -> Text,
        file_name -> Nullable<Text>,
        directory_path -> Nullable<Text>,
        icon_base64 -> Nullable<Text>,
        is_directory -> Bool,
    }
}

diesel::joinable!(file_infos -> copy_items (copy_item_id));

diesel::allow_tables_to_appear_in_same_query!(
    copy_items,
    file_infos,
);
