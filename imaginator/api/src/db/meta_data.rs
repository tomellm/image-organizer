use std::collections::HashMap;
use std::sync::Arc;

use sqlx::{Execute, MySql, Pool, QueryBuilder};
use types::database::ImageMetaData;
use types::image::MetaData;
use uuid::Uuid;

const BIND_LIMIT: usize = 65535;

pub async fn get_by_images(
    pool: Arc<Pool<MySql>>,
    uuids: &Vec<Uuid>,
) -> Result<HashMap<Uuid, Vec<ImageMetaData>>, ()> {
    get_by_str_images(
        pool,
        &uuids.iter().map(|u| u.simple().to_string()).collect(),
    )
    .await
}

pub async fn get_by_str_images(
    pool: Arc<Pool<MySql>>,
    uuids: &Vec<String>,
) -> Result<HashMap<Uuid, Vec<ImageMetaData>>, ()> {
    let str_uuids = uuids
        .iter()
        .fold(String::new(), |acc, e| format!("{acc}, {e}"));

    let out = sqlx::query_as!(
        ImageMetaData,
        "select * from images_metadata where images_uuid in (?)",
        str_uuids
    )
    .fetch_all(&*pool)
    .await
    .map_err(|err| eprintln!("ERROR: get_by_images failed to execute query. {err}"))?;

    let mut map: HashMap<Uuid, Vec<ImageMetaData>> = HashMap::new();

    //TODO: properly handle the unwrap here!
    out.into_iter().for_each(|d| {
        let uuid = Uuid::parse_str(&d.image_uuid).unwrap();
        match map.get_mut(&uuid) {
            None => drop(map.insert(uuid, vec![d])),
            Some(vec) => drop(vec.push(d)),
        };
    });

    Ok(map)
}

pub async fn get_by_image(
    pool: Arc<Pool<MySql>>,
    uuid: Uuid
) -> Result<Vec<ImageMetaData>, ()> {
    let out = sqlx::query_as!(
        ImageMetaData,
        "select * from images_metadata where image_uuid = ?",
        uuid.simple().to_string()
    )
    .fetch_all(&*pool)
    .await
    .map_err(|err| eprintln!("ERROR: get_by_image failed to execute query. {err}"))?;

    Ok(out)
}

pub async fn save_many(
    pool: Arc<Pool<MySql>>,
    meta_data: Vec<ImageMetaData>,
) -> Result<Vec<ImageMetaData>, ()> {
    if meta_data.len() == 0 {
        return Ok(meta_data);
    }

    let mut query_builder: QueryBuilder<MySql> = QueryBuilder::new(
        "insert into images_metadata (uuid, image_uuid, data_key, data_val)"
    );

    query_builder.push_values(meta_data.iter().take(BIND_LIMIT / 4), |mut b, meta| {
        b.push_bind(meta.uuid.clone());
        b.push_bind(meta.image_uuid.clone());
        b.push_bind(meta.data_key.clone());
        b.push_bind(meta.data_val.clone());
    });

    let query = query_builder.build();
    println!("This is the query beeing executed: \n {} \n", query.sql());
    query
        .execute(&*pool)
        .await
        .map_err(|err| eprintln!("ERROR: save_ failed to execute query. {err}"))?;

    Ok(meta_data)
}

pub async fn save_one(
    pool: Arc<Pool<MySql>>,
    meta_data: ImageMetaData,
) -> Result<ImageMetaData, ()> {
    let _ = sqlx::query!(
        r#"
        insert into images_metadata
            (uuid, data_key, data_val)
        values
            (?, ?, ?);
        "#,
        meta_data.uuid,
        meta_data.data_key,
        meta_data.data_val
    )
    .execute(&*pool)
    .await
    .map_err(|err| eprintln!("ERROR: save_one failed to execute query. {err}"))?;

    Ok(meta_data)
}
