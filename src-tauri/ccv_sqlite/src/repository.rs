use crate::{
    enum_converter::{from_copy_category, to_copy_category},
    model::{CopyItemEntity, FileInfoEntity, NewCopyItemEntity, NewFileInfoEntity},
};
use ccv_contract::{
    app_error,
    error::AppError,
    models::{
        CopyCategory::{self, Files},
        CopyItem, CopyItemValue, FileInfo, SearchResult,
    },
    repository::Repository,
};
use chrono::{DateTime, Utc};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use diesel::{associations::HasTable, prelude::*};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::sync::Mutex;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub struct SqliteRepository {
    pool: Mutex<Pool<ConnectionManager<SqliteConnection>>>,
}

impl SqliteRepository {
    pub fn new(database_url: &str) -> Result<Self, AppError> {
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let connection_pool = Pool::builder()
            .build(manager)
            .map_err(|err| app_error!("{}", err))?;

        let ret = SqliteRepository {
            pool: Mutex::new(connection_pool),
        };
        ret.apply_migration()?;

        Ok(ret)
    }

    pub fn apply_migration(&self) -> Result<(), AppError> {
        let pool = self.pool.lock().unwrap();
        let mut connection = pool
            .get()
            .map_err(|err| app_error!("Unable to get connection. {}", err))?;

        connection
            .run_pending_migrations(MIGRATIONS)
            .map_err(|err| app_error!("Failed to run database migrations. {}", err))?;

        Ok(())
    }

    fn to_copy_item(
        copy_item_entity: &CopyItemEntity,
        file_info_entities: &Vec<FileInfoEntity>,
    ) -> CopyItem {
        let id: String = copy_item_entity.id.to_string();
        let created = copy_item_entity.created.and_utc();
        let last_reuse = copy_item_entity.last_reuse.and_utc();
        let copy_category = to_copy_category(&copy_item_entity.category);

        let files: Option<Vec<FileInfo>> = if copy_category == Files {
            Some(
                file_info_entities
                    .iter()
                    .filter(|f| f.copy_item_id == copy_item_entity.id)
                    .map(|f| SqliteRepository::to_file_info(f))
                    .collect::<Vec<FileInfo>>(),
            )
        } else {
            None
        };

        CopyItem {
            id: id,
            created: created,
            last_reuse: last_reuse,
            value: CopyItemValue {
                category: copy_category,
                html: copy_item_entity.html.clone(),
                rtf: copy_item_entity.rtf.clone(),
                text: copy_item_entity.text.clone(),
                image: copy_item_entity.image.clone(),
                files: files,
            },
        }
    }

    fn to_file_info(file_info_entitiy: &FileInfoEntity) -> FileInfo {
        FileInfo {
            directory_path: file_info_entitiy.directory_path.clone(),
            file_name: file_info_entitiy.file_name.clone(),
            full_path: file_info_entitiy.full_path.clone(),
            icon_base64: file_info_entitiy.icon_base64.clone(),
            is_directory: file_info_entitiy.is_directory,
        }
    }

    fn squash_files(files: &Vec<FileInfo>) -> String {
        let files_full_pathes: Vec<String> = files.iter().map(|f| f.full_path.clone()).collect();
        files_full_pathes.join("\n")
    }
}

impl Repository for SqliteRepository {
    fn search(
        &self,
        query: Option<&str>,
        page: i32,
        page_size: i32,
        categories: Vec<CopyCategory>,
    ) -> Result<SearchResult, AppError> {
        let page: i64 = i64::from(page);
        if page < 0 {
            return Err(app_error!("Page cannot be negative but was {page}"));
        }
        let page_size = i64::from(page_size);
        if page_size <= 0 {
            return Err(app_error!(
                "Page size cannot be zero or negative but was {page_size}"
            ));
        }

        let pool = self.pool.lock().unwrap();
        let mut connection = pool
            .get()
            .map_err(|err| app_error!("Unable to get connection. {}", err))?;

        let categories: Vec<&'static str> =
            categories.iter().map(|c| from_copy_category(*c)).collect();

        use crate::schema::copy_items::dsl::{category, copy_items, files_text, last_reuse, text};
        let get_query = || {
            let result = copy_items.into_boxed().filter(category.eq_any(&categories));

            if let Some(q) = query {
                return result.filter(
                    text.like(format!("%{q}%"))
                        .or(files_text.like(format!("%{q}%"))),
                );
            }

            return result;
        };

        let copy_item_entities: Vec<CopyItemEntity> = get_query()
            .offset(page_size * page)
            .order_by(last_reuse.desc())
            .limit(page_size)
            .load(&mut connection)
            .map_err(|err| app_error!("Failed to query CopyItems. {err}"))?;

        let total_items: i64 = get_query()
            .count()
            .get_result(&mut connection)
            .map_err(|err| app_error!("Failed to query CopyItems count. {err}"))?;

        let file_info_entities: Vec<FileInfoEntity> =
            FileInfoEntity::belonging_to(&copy_item_entities[..])
                .select(FileInfoEntity::as_select())
                .load(&mut connection)
                .map_err(|err| app_error!("Failed to query FileInfos. {err}"))?;

        let result: Vec<CopyItem> = copy_item_entities
            .iter()
            .map(|c| SqliteRepository::to_copy_item(&c, &file_info_entities))
            .collect();

        Ok(SearchResult {
            items: result,
            total_number: usize::try_from(total_items)
                .map_err(|err| app_error!("Failed to convert total items {err}"))?,
        })
    }

    fn insert(&self, item_value: &CopyItemValue) -> Result<CopyItem, AppError> {
        let pool = self.pool.lock().unwrap();
        let mut connection = pool
            .get()
            .map_err(|err| app_error!("Unable to get connection. {}", err))?;

        let date = Utc::now();
        let files_squashed_text = if let Some(files) = &item_value.files {
            Some(SqliteRepository::squash_files(&files))
        } else {
            None
        };

        let new_copy_item_entity = NewCopyItemEntity {
            created: date.naive_utc(),
            last_reuse: date.naive_utc(),
            category: from_copy_category(item_value.category),
            text: item_value.text.as_ref().map(|t| t.as_str()),
            html: item_value.html.as_ref().map(|h| h.as_str()),
            rtf: item_value.rtf.as_ref().map(|r| r.as_str()),
            image: item_value.image.as_ref().map(|i| i.as_str()),
            files_text: files_squashed_text.as_ref().map(|f| f.as_str()),
        };

        use crate::schema::copy_items::dsl::copy_items;
        let inserted_copy_item_entity: CopyItemEntity = diesel::insert_into(copy_items)
            .values(&new_copy_item_entity)
            .returning(CopyItemEntity::as_select())
            .get_result(&mut connection)
            .map_err(|err| app_error!("Failed to insert CopyItems. {err}"))?;

        let inserted_file_info_entities: Vec<FileInfoEntity> =
            if let Some(files) = &item_value.files {
                let mut result: Vec<FileInfoEntity> = vec![];

                use crate::schema::file_infos::dsl::file_infos;
                for file in files {
                    let new_file_info_entity = NewFileInfoEntity {
                        copy_item_id: inserted_copy_item_entity.id,
                        directory_path: file.directory_path.as_ref().map(|d_path| d_path.as_str()),
                        file_name: file.file_name.as_ref().map(|f_name| f_name.as_str()),
                        full_path: file.full_path.as_ref(),
                        icon_base64: file.icon_base64.as_ref().map(|icon| icon.as_str()),
                        is_directory: file.is_directory,
                    };

                    let inserted_file_info_entity: FileInfoEntity = diesel::insert_into(file_infos)
                        .values(&new_file_info_entity)
                        .returning(FileInfoEntity::as_select())
                        .get_result(&mut connection)
                        .map_err(|err| app_error!("Failed to insert FileInfo. {err}"))?;

                    result.push(inserted_file_info_entity);
                }

                result
            } else {
                vec![]
            };

        Ok(SqliteRepository::to_copy_item(
            &inserted_copy_item_entity,
            &inserted_file_info_entities,
        ))
    }

    fn find_by_value(&self, item_value: &CopyItemValue) -> Result<Option<CopyItem>, AppError> {
        let pool = self.pool.lock().unwrap();
        let mut connection = pool
            .get()
            .map_err(|err| app_error!("Unable to get connection. {}", err))?;

        use crate::schema::copy_items::dsl::*;

        let mut copy_items_table = copy_items::table().into_boxed();

        if let Some(text_value) = &item_value.text {
            copy_items_table = copy_items_table.filter(text.is_not_null().and(text.eq(text_value)));
        } else {
            copy_items_table = copy_items_table.filter(text.is_null());
        };
        if let Some(html_value) = &item_value.html {
            copy_items_table = copy_items_table.filter(html.is_not_null().and(html.eq(html_value)));
        } else {
            copy_items_table = copy_items_table.filter(html.is_null());
        };
        if let Some(rtf_value) = &item_value.rtf {
            copy_items_table = copy_items_table.filter(rtf.is_not_null().and(rtf.eq(rtf_value)));
        } else {
            copy_items_table = copy_items_table.filter(rtf.is_null());
        };
        if let Some(image_value) = &item_value.image {
            copy_items_table =
                copy_items_table.filter(image.is_not_null().and(image.eq(image_value)));
        } else {
            copy_items_table = copy_items_table.filter(image.is_null());
        };
        if let Some(files_value) = &item_value.files {
            let squashed_files = SqliteRepository::squash_files(files_value);
            copy_items_table = copy_items_table
                .filter(files_text.is_not_null().and(files_text.eq(squashed_files)));
        } else {
            copy_items_table = copy_items_table.filter(files_text.is_null());
        };

        copy_items_table =
            copy_items_table.filter(category.eq(from_copy_category(item_value.category)));

        let copy_item_entities: Vec<CopyItemEntity> = copy_items_table
            .select(CopyItemEntity::as_select())
            .get_results(&mut connection)
            .map_err(|err| app_error!("Failed to fetch CopyItems. {err}"))?;

        if copy_item_entities.is_empty() {
            Ok(None)
        } else {
            let copy_item_entity = copy_item_entities.into_iter().next().unwrap();
            let file_info_entities: Vec<FileInfoEntity> =
                FileInfoEntity::belonging_to(&copy_item_entity)
                    .select(FileInfoEntity::as_select())
                    .load(&mut connection)
                    .map_err(|err| app_error!("Failed to query FileInfos. {err}"))?;
            Ok(Some(SqliteRepository::to_copy_item(
                &copy_item_entity,
                &file_info_entities,
            )))
        }
    }

    fn update_last_resue(
        &self,
        item_id: &str,
        new_last_reuse: DateTime<Utc>,
    ) -> Result<CopyItem, AppError> {
        let pool = self.pool.lock().unwrap();
        let mut connection = pool
            .get()
            .map_err(|err| app_error!("Unable to get connection. {}", err))?;

        let item_id_parsed = item_id
            .parse::<i32>()
            .map_err(|_| app_error!("Failed to parse item id '{}'", item_id))?;

        use crate::schema::copy_items::dsl::{copy_items, id, last_reuse};
        let updated_copy_items: Vec<CopyItemEntity> =
            diesel::update(copy_items.filter(id.eq(item_id_parsed)))
                .set(last_reuse.eq(new_last_reuse.naive_utc()))
                .returning(CopyItemEntity::as_select())
                .get_results(&mut connection)
                .map_err(|err| app_error!("Failed to update CopyItems. {}", err))?;

        if updated_copy_items.is_empty() {
            Err(app_error!(
                "No CopyItem with id '{item_id_parsed}' was found"
            ))
        } else {
            let update_copy_item = updated_copy_items.into_iter().next().unwrap();
            let updated_copy_item_file_infos: Vec<FileInfoEntity> =
                FileInfoEntity::belonging_to(&update_copy_item)
                    .select(FileInfoEntity::as_select())
                    .load(&mut connection)
                    .map_err(|err| app_error!("Failed to query FileInfos. {err}"))?;
            Ok(SqliteRepository::to_copy_item(
                &update_copy_item,
                &updated_copy_item_file_infos,
            ))
        }
    }

    fn remove(&self, item_ids: &Vec<&str>) -> Result<usize, AppError> {
        let pool = self.pool.lock().unwrap();
        let mut connection = pool
            .get()
            .map_err(|err| app_error!("Unable to get connection. {}", err))?;

        let parsed_item_ids = item_ids
            .iter()
            .map(|i| i.parse::<i32>())
            .filter(|parsed| parsed.is_ok())
            .map(|parsed| parsed.unwrap())
            .collect::<Vec<i32>>();

        use crate::schema::copy_items::dsl::{copy_items, id};
        let deleted_copy_items: Vec<CopyItemEntity> =
            diesel::delete(copy_items.filter(id.eq_any(parsed_item_ids)))
                .returning(CopyItemEntity::as_select())
                .get_results(&mut connection)
                .map_err(|err| app_error!("Failed to delete CopyItems. {}", err))?;

        Ok(deleted_copy_items.len())
    }

    fn remove_older(&self, sinse: DateTime<Utc>) -> Result<usize, AppError> {
        let pool = self.pool.lock().unwrap();
        let mut connection = pool
            .get()
            .map_err(|err| app_error!("Unable to get connection. {}", err))?;

        use crate::schema::copy_items::dsl::{copy_items, last_reuse};
        let deleted_copy_items: Vec<CopyItemEntity> =
            diesel::delete(copy_items.filter(last_reuse.le(sinse.naive_utc())))
                .returning(CopyItemEntity::as_select())
                .get_results(&mut connection)
                .map_err(|err| app_error!("Failed to delete CopyItems. {}", err))?;

        Ok(deleted_copy_items.len())
    }
}
