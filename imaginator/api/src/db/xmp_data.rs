use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {

use std::collections::HashMap;
use std::sync::Arc;

use sqlx::{MySql, Pool, QueryBuilder};
use types::database::MediaXmpData;
use uuid::Uuid;

const BIND_LIMIT: usize = 10000;
const BLOCK_LENGTH: usize = BIND_LIMIT / 4; // 4 because XmpData has 4 attributes

pub async fn get_by_images(
    pool: Arc<Pool<MySql>>,
    uuids: &Vec<Uuid>,
) -> Result<HashMap<Uuid, Vec<MediaXmpData>>, ()> {
    get_by_str_medias(
        pool,
        &uuids.iter().map(|u| u.simple().to_string()).collect(),
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
    let mut query_builder = QueryBuilder::new(
        "select * from xmpdata where media_uuid in ("
    );

    uuids.into_iter()
        .enumerate()
        .for_each(|(index, uuid)| {
            if index != 0 { query_builder.push(","); };
            query_builder.push_bind(uuid);
        });
    query_builder.push(")");

    let data = query_builder.build_query_as::<MediaXmpData>()
        .fetch_all(&*pool)
        .await
        .map_err(|err| {
        tracing::event!(
            tracing::Level::ERROR,
            "get_by_str_images failed to execute query. {}",
            err
        )
    })?;

    let mut map: HashMap<Uuid, Vec<MediaXmpData>> = HashMap::new();

    //TODO: properly handle the unwrap here!
    data.into_iter().for_each(|d| {
        let uuid = Uuid::parse_str(&d.media_uuid).unwrap();
        match map.get_mut(&uuid) {
            None => drop(map.insert(uuid, vec![d])),
            Some(vec) => drop(vec.push(d)),
        };
    });

    Ok(map)
}

pub async fn get_by_media(pool: Arc<Pool<MySql>>, uuid: Uuid) -> Result<Vec<MediaXmpData>, ()> {
    let out = sqlx::query_as!(
        MediaXmpData,
        "select * from xmpdata where media_uuid = ?",
        uuid.simple().to_string()
    )
    .fetch_all(&*pool)
    .await
    .map_err(|err| {
        tracing::event!(
            tracing::Level::ERROR,
            "ERROR: get_by_image failed to execute query. {}",
            err
        )
    })?;

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
                                "insert into xmpdata (uuid, media_uuid, data_key, data_val)",
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

pub async fn save_one(pool: Arc<Pool<MySql>>, xmp_data: MediaXmpData) -> Result<MediaXmpData, ()> {
    let _ = sqlx::query!(
        r#"
        insert into images_xmpdata
            (uuid, data_key, data_val)
        values
            (?, ?, ?);
        "#,
        xmp_data.uuid,
        xmp_data.data_key,
        xmp_data.data_val
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

    Ok(xmp_data)
}

pub async fn get_all_dates(
    pool: Arc<Pool<MySql>>
) -> Result<Vec<MediaXmpData>, ()> {
    sqlx::query_as!(
        MediaXmpData,
        r#"select * from xmpdata where data_key in
            ('photoshop:DateCreated')
        "#
        )
        .fetch_all(&*pool)
        .await
        .map_err(|err| {
            tracing::event!(
                tracing::Level::ERROR,
                "ERROR: get_all_dates failed to execute query. {}",
                err
            )
        })
}

}}
