use std::sync::Arc;

use imaginator_types::args::Pagination;
use sqlx::{MySql, Pool, QueryBuilder};
use tracing::{event, Level};
use uuid::Uuid;

use crate::{
    types::{mediatype::DBEnum, IntoDBUuid, MediaData},
    util::{add_in_items, AdapterFuture, AwaitQueryResponses, LogMysqlError},
};

use super::util;

const BIND_LIMIT: usize = 10000;
const BLOCK_LENGTH: usize = BIND_LIMIT / 6; // 4 because MetaData has 4 attributes

pub async fn save_one(pool: Arc<Pool<MySql>>, image: MediaData) -> Result<(), ()> {
    let _ = sqlx::query(
        r#"Insert into media_data 
            (uuid, original_name, current_name, extension, media_type)
        values (?,?,?,?,?)"#,
    )
    .bind(&image.uuid)
    .bind(&image.original_name)
    .bind(&image.current_name)
    .bind(&image.extension)
    .bind(&image.media_type)
    .execute(&*pool)
    .await
    .log_err("save_one failed to execute query")?;

    Ok(())
}

pub fn save_many(
    pool: Arc<Pool<MySql>>,
    images: Vec<MediaData>,
) -> impl AdapterFuture<Result<(), ()>> {
    util::query::save_many(pool, images)
}

pub async fn get_all(pool: Arc<Pool<MySql>>) -> Result<Vec<MediaData>, ()> {
    let out = sqlx::query_as("select * from media_data")
        .fetch_all(&*pool)
        .await
        .log_err("get_all failed to execute query")?;
    Ok(out)
}

pub async fn get_many(pool: Arc<Pool<MySql>>, uuids: Vec<Uuid>) -> Result<Vec<MediaData>, ()> {
    let mut query_builder = QueryBuilder::new("select * from media_data where uuid in (");
    uuids.into_iter().enumerate().for_each(|(index, uuid)| {
        if index != 0 {
            query_builder.push(",");
        };
        query_builder.push_bind(uuid.into_db());
    });
    query_builder.push(")");

    let sql = query_builder.build_query_as::<MediaData>();

    sql.fetch_all(&*pool)
        .await
        .log_err("get_many failed to execute query")
}

pub async fn get_all_images(pool: Arc<Pool<MySql>>) -> Result<Vec<MediaData>, ()> {
    let mut query_builder = util::add_in_items(
        "select * from media_data where media_type in (",
        DBEnum::get_image_types_u8().into_iter(),
        ")",
    );
    /*
    let mut query_builder = QueryBuilder::new(
        "select * from media_data where media_type in ("
    );

    MediaType::get_image_types_u8().into_iter()
        .enumerate()
        .for_each(|(index, id)|{
            if index != 0 { query_builder.push(","); };
            query_builder.push_bind(id);
        });
    query_builder.push(")");*/

    query_builder
        .build_query_as::<MediaData>()
        .fetch_all(&*pool)
        .await
        .log_err("get_all failed to execute query")
}

pub async fn get_images_paginated(
    pool: Arc<Pool<MySql>>,
    page: Pagination,
) -> Result<Option<Vec<MediaData>>, ()> {
    let (offset, limit) = page.get_vals();

    let num = util::get_number_images(pool.clone()).await?;

    if num < offset {
        return Ok(None);
    }

    let mut query_builder = util::add_in_items(
        "select * from media_data where media_type in (",
        DBEnum::get_image_types_u8(),
        ") order by datetime_created asc limit ",
    );
    query_builder.push_bind(limit);
    query_builder.push(" offset ");
    query_builder.push_bind(offset);

    event!(Level::DEBUG, "sql is: {:?}", query_builder.sql());

    query_builder
        .build_query_as::<MediaData>()
        .fetch_all(&*pool)
        .await
        .log_err("get_all failed to execute query")
        .map(|v| Some(v))
}

pub async fn get_one(pool: Arc<Pool<MySql>>, uuid: Uuid) -> Result<MediaData, ()> {
    let out = sqlx::query_as("select * from media_data where uuid = ?")
        .bind(uuid.into_db())
        .fetch_one(&*pool)
        .await
        .log_err("get_one failed to execute query")?;

    Ok(out)
}

pub async fn delete_all(pool: Arc<Pool<MySql>>) -> Result<(), ()> {
    let _ = sqlx::query("truncate table media_data")
        .execute(&*pool)
        .await
        .log_err("delete_all failed to execute")?;

    Ok(())
}

pub fn delete_many(pool: Arc<Pool<MySql>>, keys: Vec<Uuid>) -> impl AdapterFuture<Result<(), ()>> {
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
                        add_in_items("delete from media_data where uuid in (", keys_chunk, ");");
                    query_builder.build().execute(&*new_pool).await
                }
            })
            .collect::<Vec<_>>()
            .join_await()
            .await
    }
}
