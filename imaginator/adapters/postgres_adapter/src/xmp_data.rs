use std::collections::HashMap;
use std::sync::Arc;

use sqlx::{MySql, Pool, QueryBuilder};
use uuid::Uuid;

use crate::{types::{xmpdata::MediaXmpData, FromDBUuid, IntoDBUuid}, util::LogMysqlError};

const BIND_LIMIT: usize = 10000;
const BLOCK_LENGTH: usize = BIND_LIMIT / 4; // 4 because XmpData has 4 attributes

pub async fn get_by_images(
    pool: Arc<Pool<MySql>>,
    uuids: &Vec<Uuid>,
) -> Result<HashMap<Uuid, Vec<MediaXmpData>>, ()> {
    get_by_str_medias(
        pool,
        &uuids.iter().map(|u| u.into_db()).collect(),
    )
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
) -> Result<HashMap<Uuid, Vec<MediaXmpData>>, ()> {
    let mut query_builder = QueryBuilder::new("select * from xmp_data where media_uuid in (");

    uuids.into_iter().enumerate().for_each(|(index, uuid)| {
        if index != 0 {
            query_builder.push(",");
        };
        query_builder.push_bind(uuid);
    });
    query_builder.push(")");

    let data = query_builder
        .build_query_as::<MediaXmpData>()
        .fetch_all(&*pool)
        .await
        .log_err("get_by_str_images failed to execute query")?;

    let mut map: HashMap<Uuid, Vec<MediaXmpData>> = HashMap::new();

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

pub async fn get_by_media(pool: Arc<Pool<MySql>>, uuid: Uuid) -> Result<Vec<MediaXmpData>, ()> {
    let out = sqlx::query_as("select * from xmp_data where media_uuid = ?")
        .bind(uuid.into_db())
        .fetch_all(&*pool)
        .await
        .log_err("get_by_image failed to execute query")?;

    Ok(out)
}

pub async fn save_many(pool: Arc<Pool<MySql>>, xmp_data: Vec<MediaXmpData>) -> Result<(), ()> {
    if xmp_data.len() == 0 {
        return Ok(());
    }

    let mut chunks = xmp_data.into_iter().enumerate().fold(
        vec![],
        |mut acc: Vec<(QueryBuilder<MySql>, Vec<MediaXmpData>)>,
         (pos, data): (usize, MediaXmpData)| {
            let index = (pos as f32 / BLOCK_LENGTH as f32).floor() as usize;
            let inner_index = (pos as f32 % BLOCK_LENGTH as f32) as usize;
            match acc.get_mut(index) {
                Some(inner_vec) => {
                    inner_vec.1.insert(inner_index, data);
                }
                None => {
                    acc.insert(
                        index,
                        (
                            QueryBuilder::new(
                                "insert into xmp_data (uuid, media_uuid, data_key, data_val)",
                            ),
                            vec![],
                        ),
                    );
                    let inner_vec = acc.get_mut(index).unwrap();
                    inner_vec.1.insert(inner_index, data);
                }
            }
            acc
        },
    );
    let mut futures = vec![];
    for (ref mut query_builder, chunk) in chunks.iter_mut() {
        query_builder.push_values(chunk, |mut b, xmp| {
            b.push_bind(xmp.uuid.clone());
            b.push_bind(xmp.media_uuid.clone());
            b.push_bind(xmp.data_key.clone());
            b.push_bind(xmp.data_val.clone());
        });

        let query = query_builder.build();
        futures.push(query.execute(&*pool));
    }
    for future in futures {
        future.await.log_err("save_many failed to execute query")?;
    }

    Ok(())
}

pub async fn save_one(pool: Arc<Pool<MySql>>, xmp_data: MediaXmpData) -> Result<MediaXmpData, ()> {
    let _ = sqlx::query(
        r#"
        insert into images_xmp_data
            (uuid, data_key, data_val)
        values
            (?, ?, ?);
        "#,
    )
    .bind(&xmp_data.uuid)
    .bind(&xmp_data.data_key)
    .bind(&xmp_data.data_val)
    .execute(&*pool)
    .await
    .log_err("save_one failed to execute query")?;

    Ok(xmp_data)
}

pub async fn get_all_dates(pool: Arc<Pool<MySql>>) -> Result<Vec<MediaXmpData>, ()> {
    sqlx::query_as(
        r#"select * from xmp_data where data_key in
            ('photoshop:DateCreated')
        "#
    )
    .fetch_all(&*pool)
    .await
    .log_err("get_all_dates failed to execute query") 
}

pub async fn delete_all(pool: Arc<Pool<MySql>>) -> Result<(), ()> {
    let _ = sqlx::query("truncate table xmp_data")
        .execute(&*pool)
        .await
        .log_err("delete_all failed to execute")?;

    Ok(())
}
