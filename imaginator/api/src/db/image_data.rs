use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {

use std::sync::Arc;
use crate::db::util;

use sqlx::{MySql, Pool, QueryBuilder, query_builder, Execute};
use tracing::{Level, event};
use types::{database::MediaData, mediatypes::MediaType, args::Pagination};
use uuid::Uuid;

const BIND_LIMIT: usize = 10000;
const BLOCK_LENGTH: usize = BIND_LIMIT / 6; // 4 because MetaData has 4 attributes

pub async fn save_one(pool: Arc<Pool<MySql>>, image: MediaData) -> Result<(), ()> {
    let _ = sqlx::query!(
        r#"Insert into media_data 
            (uuid, original_name, current_name, extension, media_type)
        values (?,?,?,?,?)"#,
        image.uuid,
        image.original_name,
        image.current_name,
        image.extension,
        image.media_type
    )
    .execute(&*pool)
    .await
    .map_err(|err| {
        tracing::event!(
            tracing::Level::ERROR,
            "ERROR: save_one failed to execute query. {}",
            err
        )
    })?;

    Ok(())
}

pub async fn save_many(pool: Arc<Pool<MySql>>, images: Vec<MediaData>) -> Result<(), ()> {
    if images.len() == 0 {
        return Ok(());
    }

    let mut chunks = images.into_iter().enumerate()
        .fold(vec![], |mut acc: Vec<(QueryBuilder<MySql>, Vec<MediaData>)>, (pos, data): (usize, MediaData)| {
            let index = (pos as f32 / BLOCK_LENGTH as f32).floor() as usize;
            let inner_index = (pos as f32 % BLOCK_LENGTH as f32) as usize;
            match acc.get_mut(index) {
                Some(inner_vec) => {
                    inner_vec.1.insert(inner_index, data);
                },
                None => {
                    acc.insert(index, (
                            QueryBuilder::new(
                                r#"
                                insert into media_data 
                                    (
                                        uuid, original_name, current_name, 
                                        extension, media_type, datetime_created
                                    )
                                "#),
                            vec![]
                    ));
                    let inner_vec = acc.get_mut(index).unwrap();
                    inner_vec.1.insert(inner_index, data);
                }
            }
            acc
        });
    let mut futures = vec![];
    for (ref mut query_builder, chunk) in chunks.iter_mut() {
        query_builder.push_values(chunk, |mut b, img| {
            b.push_bind(img.uuid.clone());
            b.push_bind(img.original_name.clone());
            b.push_bind(img.current_name.clone());
            b.push_bind(img.extension.clone());
            b.push_bind(img.media_type.clone());
            b.push_bind(img.datetime_created.clone());
        });

        let query = query_builder.build();
        futures.push(query.execute(&*pool));
    }
    for future in futures {
        future.await.map_err(|err| {
            tracing::event!(
                tracing::Level::ERROR,
                "ERROR: save_many failed to execute query. {}",
                err
            )
        })?;
    }

    Ok(())
}

pub async fn get_all(pool: Arc<Pool<MySql>>) -> Result<Vec<MediaData>, ()> {
    let out = sqlx::query_as!(MediaData, "select * from media_data")
        .fetch_all(&*pool)
        .await
        .map_err(|err| {
            tracing::event!(
                tracing::Level::ERROR,
                "ERROR: get_all failed to execute query. {}",
                err
            )
        })?;
    Ok(out)
}

pub async fn get_many(
    pool: Arc<Pool<MySql>>,
    uuids: Vec<Uuid>
) -> Result<Vec<MediaData>, ()> {

    let mut query_builder = QueryBuilder::new(
        "select * from media_data where uuid in ("
    );
    uuids.into_iter()
        .enumerate()
        .for_each(|(index, uuid)| {
            if index != 0 { query_builder.push(",");};
            query_builder.push_bind(uuid.simple().to_string());
        });
    query_builder.push(")");
    
    let sql = query_builder.build_query_as::<MediaData>();
    
    sql.fetch_all(&*pool)
        .await
        .map_err(|err| {
            tracing::event!(
                tracing::Level::DEBUG,
                "get_many failed to execute query {}", err
            )
        })
}

pub async fn get_all_images(
    pool: Arc<Pool<MySql>>
) -> Result<Vec<MediaData>, ()> {

    let mut query_builder = util::add_in_items(
        "select * from media_data where media_type in (",
        MediaType::get_image_types_u8().into_iter(),
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

    query_builder.build_query_as::<MediaData>()
        .fetch_all(&*pool)
        .await
        .map_err(|err| {
            tracing::event!(
                tracing::Level::ERROR,
                "ERROR: get_all failed to execute query. {}", err
            )
        })
}

pub async fn get_images_paginated(
    pool: Arc<Pool<MySql>>,
    page: Pagination,
) -> Result<Option<Vec<MediaData>>, ()> {
    let (offset, limit) = page.get_vals();

    let num = util::get_number_images(pool.clone()).await?;

    if num < offset {
        return Ok(None)
    }



    let mut query_builder = util::add_in_items(
        "select * from media_data where media_type in (",
        MediaType::get_image_types_u8().into_iter(),
        ") order by datetime_created asc limit "
    );
    query_builder.push_bind(limit);
    query_builder.push(" offset ");
    query_builder.push_bind(offset);

    event!(Level::DEBUG, "sql is: {:?}", query_builder.sql());


    query_builder.build_query_as::<MediaData>()
        .fetch_all(&*pool)
        .await
        .map_err(|err| {
            tracing::event!(
                tracing::Level::ERROR,
                "ERROR: get_all failed to execute query. {}", err
            )
        })
        .map(|v|Some(v))

}

pub async fn get_one(pool: Arc<Pool<MySql>>, uuid: Uuid) -> Result<MediaData, ()> {
    let out = sqlx::query_as!(
        MediaData,
        "select * from media_data where uuid = ?",
        uuid.simple().to_string()
    )
    .fetch_one(&*pool)
    .await
    .map_err(|err| {
        tracing::event!(
            tracing::Level::ERROR,
            "ERROR: get_one failed to execute query. {}",
            err
        )
    })?;

    Ok(out)
}

}}
