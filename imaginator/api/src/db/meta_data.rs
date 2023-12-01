use std::collections::HashMap;
use std::sync::Arc;

use sqlx::{Pool, MySql};
use tracing_subscriber::fmt::format;
use types::database::ImageMetaData;
use types::image::MetaData;
use uuid::Uuid;

pub async fn get_by_images(
    pool: Arc<Pool<MySql>>,
    uuids: &Vec<Uuid>
) -> Result<HashMap<Uuid, Vec<ImageMetaData>>, ()> {
    get_by_str_images(
        pool,
        &uuids.iter().map(|u| u.simple().to_string()).collect()
    ).await
}

pub async fn get_by_str_images(
    pool: Arc<Pool<MySql>>,
    uuids: &Vec<String>
) -> Result<HashMap<Uuid, Vec<ImageMetaData>>, ()> {

    let str_uuids = uuids.iter()
        .fold(
            String::new(), 
            |acc, e| format!("{acc}, {e}")
        );

    let out = sqlx::query_as!(
        ImageMetaData,
        "select * from image_metadata where image_uuid in (?)",
        str_uuids
    )
        .fetch_all(&*pool).await
        .map_err(|err| eprintln!("ERROR: get_by_images failed to execute query. {err}"))?;

    let mut map: HashMap<Uuid, Vec<ImageMetaData>> = HashMap::new();

    //TODO: properly handle the unwrap here!
    out.into_iter()
        .for_each(|d| {
            let uuid = Uuid::parse_str(&d.image_uuid).unwrap();
            match map.get_mut(&uuid) {
                None => drop(map.insert(uuid, vec![d])),
                Some(vec) => drop(vec.push(d))
            };
        });

    Ok(map)
}



pub async fn get_by_image(
    pool: Arc<Pool<MySql>>,
    uuid: Uuid
) -> Result<Vec<MetaData>, ()> {

    let out = sqlx::query_as!(
        ImageMetaData,
        "select * from image_metadata where image_uuid = ?",
        uuid.simple().to_string()
    )
        .fetch_all(&*pool)
        .await
        .map_err(|err| eprintln!("ERROR: get_by_image failed to execute query. {err}"))?;

    //TODO: properly handle the unwrap here!
    Ok(out.into_iter().map(|d| d.to_struct().unwrap()).collect()) 
}

pub async fn save_many(
    pool: Arc<Pool<MySql>>,
    meta_data: Vec<ImageMetaData>,
) -> Result<Vec<ImageMetaData>, ()> {
    let mut query_builder: QueryBuilder<MySql> = QueryBuilder::new(
        r#"
            insert into images_metadata
                (uuid, data_key, data_val)
            values
                (?, ?, ?);
        "#,
    );
    // One element vector is handled correctly but an empty vector
    // would cause a sql syntax error
    let mut separated = query_builder.separated(", ");
    for value_type in foods.iter() {
      separated.push_bind(value_type);
    }
    separated.push_unseparated(") ");

    let mut query = query_builder.build();
    let sql = query.sql();
    assert!(sql.ends_with("in (?, ?) "));

    let values = meta_data.into_iter()
        .map(|d| format!("({}, {}, {})", d.uuid, d.data_key, d.data_val))
        .fold(String::new(), |acc, e| format!("{}, {}", acc, )
}

pub async fn save_one(
    pool: Arc<Pool<MySql>>,
    meta_data: ImageMetaData
) -> Result<ImageMetaData, ()> {
    let _ = sqlx::query!(
        r#"
        insert into images_metadata
            (uuid, data_key, data_val)
        values
            (?, ?, ?);
        "#,
        meta_data.uuid, meta_data.data_key, meta_data.data_val
    )
        .execute(&*pool).await
        .map_err(|err| eprintln!("ERROR: save_one failed to execute query. {err}"))?;

    Ok(meta_data)
}
