use std::collections::HashMap;
use std::sync::Arc;

use sqlx::{Pool, MySql};
use types::database::ImageXmpData;
use types::image::XmpData;
use uuid::Uuid;


pub async fn get_by_images(
    pool: Arc<Pool<MySql>>,
    uuids: &Vec<Uuid>
) -> Result<HashMap<Uuid, Vec<ImageXmpData>>, ()> {
    get_by_str_images(
        pool,
        &uuids.iter().map(|u| u.simple().to_string()).collect()
    ).await
}

pub async fn get_by_str_images(
    pool: Arc<Pool<MySql>>,
    uuids: &Vec<String>
) -> Result<HashMap<Uuid, Vec<ImageXmpData>>, ()> {

    let str_uuids = uuids.iter()
        .fold(
            String::new(), 
            |acc, e| format!("{acc}, {e}")
        );

    let out = sqlx::query_as!(
        ImageXmpData,
        "select * from image_xmpdata where image_uuid in (?)",
        str_uuids
    )
        .fetch_all(&*pool).await
        .map_err(|err| eprintln!("ERROR: get_by_images failed to execute query. {err}"))?;

    let mut map: HashMap<Uuid, Vec<ImageXmpData>> = HashMap::new();

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
) -> Result<Vec<XmpData>, ()> {

    let out = sqlx::query_as!(
        ImageXmpData,
        "select * from image_xmpdata where image_uuid = ?",
        uuid.simple().to_string()
    )
        .fetch_all(&*pool)
        .await
        .map_err(|err| eprintln!("ERROR: get_by_image failed to execute query. {err}"))?;

    //TODO: properly handle the unwrap here!
    Ok(out.into_iter().map(|d| d.to_struct().unwrap()).collect()) 
}
