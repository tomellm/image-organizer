use std::collections::HashMap;
use std::sync::Arc;

use sqlx::{MySql, Pool, QueryBuilder};
use uuid::Uuid;

use crate::{
    types::{metadata::MediaMetaData, FromDBUuid, IntoDBUuid},
    util::{self, add_in_items, AdapterFuture, AwaitQueryResponses, LogMysqlError},
};

const BIND_LIMIT: usize = 10000;
const BLOCK_LENGTH: usize = BIND_LIMIT / 4; // 4 because MetaData has 4 attributes

pub async fn get_by_images(
    pool: Arc<Pool<MySql>>,
    uuids: &Vec<Uuid>,
) -> Result<HashMap<Uuid, Vec<MediaMetaData>>, ()> {
    get_by_str_medias(pool, &uuids.iter().map(|u| u.into_db()).collect())
        .await
        .map_err(|_| {
            tracing::event!(
                tracing::Level::ERROR,
                "get_by_images failed to execute query."
            )
        })
}

pub async fn get_by_str_medias(
    pool: Arc<Pool<MySql>>,
    uuids: &Vec<String>,
) -> Result<HashMap<Uuid, Vec<MediaMetaData>>, ()> {
    let mut query_builder = QueryBuilder::new("select * from meta_data where media_uuid in (");

    uuids.into_iter().enumerate().for_each(|(index, uuid)| {
        if index != 0 {
            query_builder.push(",");
        };
        query_builder.push_bind(uuid);
    });
    query_builder.push(")");

    let data = query_builder
        .build_query_as::<MediaMetaData>()
        .fetch_all(&*pool)
        .await
        .log_err("get_by_str_images failed to execute query")?;
    let mut map: HashMap<Uuid, Vec<MediaMetaData>> = HashMap::new();

    //TODO: properly handle the unwrap here!
    data.into_iter().for_each(|d| {
        let uuid = Uuid::from_db(&d.media_uuid).unwrap();
        match map.get_mut(&uuid) {
            None => drop(map.insert(uuid, vec![d])),
            Some(vec) => drop(vec.push(d)),
        };
    });

    Ok(map)
}

pub async fn get_by_media(pool: Arc<Pool<MySql>>, uuid: Uuid) -> Result<Vec<MediaMetaData>, ()> {
    let out = sqlx::query_as("select * from meta_data where media_uuid = ?")
        .bind(uuid.into_db())
        .fetch_all(&*pool)
        .await
        .log_err("get_by_image failed to execute query")?;

    Ok(out)
}

pub fn save_many(
    pool: Arc<Pool<MySql>>,
    meta_data: Vec<MediaMetaData>,
) -> impl AdapterFuture<Result<(), ()>> {
    util::query::save_many(pool, meta_data)
}

pub async fn save_one(
    pool: Arc<Pool<MySql>>,
    meta_data: MediaMetaData,
) -> Result<MediaMetaData, ()> {
    let _ = sqlx::query(
        r#"
        insert into meta_data
            (uuid, data_key, data_val)
        values
            (?, ?, ?);
        "#,
    )
    .bind(&meta_data.uuid)
    .bind(&meta_data.data_key)
    .bind(&meta_data.data_val)
    .execute(&*pool)
    .await
    .log_err("save_one failed to execute query")?;

    Ok(meta_data)
}

pub async fn get_all_dates(pool: Arc<Pool<MySql>>) -> Result<Vec<MediaMetaData>, ()> {
    sqlx::query_as(
        r#"select * from meta_data where data_key in 
            ('DateTimeOriginal', 'DateTime', 'DateTimeDigitized')
        "#,
    )
    .fetch_all(&*pool)
    .await
    .log_err("get_all_dates failed to execute query")
}

pub async fn delete_all(pool: Arc<Pool<MySql>>) -> Result<(), ()> {
    let _ = sqlx::query("truncate table meta_data")
        .execute(&*pool)
        .await
        .log_err("delete_all failed to execute")?;

    Ok(())
}

pub fn delete_many_by_media(pool: Arc<Pool<MySql>>, keys: Vec<Uuid>) -> impl AdapterFuture<Result<(), ()>> {
    async move {
        if keys.len() == 0 {
            return Ok(());
        }
        keys.chunks(BIND_LIMIT)
            .map(|chunk| {
                let keys_chunk = chunk
                    .into_iter()
                    .map(|uuid| uuid.into_db())
                    .collect::<Vec<_>>();
                let new_pool = pool.clone();
                async move {
                    let mut query_builder =
                        add_in_items("delete from meta_data where media_uuid in (", keys_chunk, ");");
                    query_builder.build().execute(&*new_pool).await
                }
            })
            .collect::<Vec<_>>()
            .join_await()
            .await
    }
}
