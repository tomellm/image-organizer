mod meta_data;
mod xmp_data;

use std::sync::Arc;

use sqlx::{pool, MySql, Pool, QueryBuilder};
use types::{database::ImageData, image::Image};
use uuid::Uuid;

pub async fn save_image(pool: Arc<Pool<MySql>>, image: Image) -> Result<(), ()> {
    let image = image.to_db().0;

    let out = sqlx::query!(
        r#"Insert into images_data 
            (uuid, original_name, current_name, extension)
        values (?,?,?,?)"#,
        image.uuid,
        image.original_name,
        image.current_name,
        image.extension
    )
    .execute(&*pool)
    .await
    .expect("could not fetch");

    println!("{out:?}");

    Ok(())
}

pub async fn get_images(pool: Arc<Pool<MySql>>) -> Result<Vec<Image>, ()> {
    let out = sqlx::query_as!(ImageData, "select * from images_data")
        .fetch_all(&*pool)
        .await
        .map_err(|err| eprintln!("ERROR: get_images failed to execute query. {err}"))?;

    let all_uuids = out.iter().map(|i| i.uuid.clone()).collect::<Vec<_>>();

    let mut xmp_data = xmp_data::get_by_str_images(pool.clone(), &all_uuids)
        .await
        .unwrap();
    let mut meta_data = meta_data::get_by_str_images(pool.clone(), &all_uuids)
        .await
        .unwrap();

    Ok(out
        .into_iter()
        .map(|id| {
            let uuid = Uuid::parse_str(&id.uuid).unwrap();
            let xmp = xmp_data.remove(&uuid).unwrap_or(vec![]);
            let meta = meta_data.remove(&uuid).unwrap_or(vec![]);
            id.to_struct(meta, xmp)
        })
        .collect())
}

pub async fn get_image(pool: Arc<Pool<MySql>>, uuid: Uuid) -> Result<Image, ()> {
    let out = sqlx::query_as!(
        ImageData,
        "select * from images_data where uuid = ?",
        uuid.simple().to_string()
    )
    .fetch_one(&*pool)
    .await
    .map_err(|err| eprintln!("ERROR: get_image failed to execute query {err}"))?;
    Ok(out.to_struct(vec![], vec![]))
}

/*
pub async fn get_images(pool: Arc<Pool<MySql>>) -> Vec<Image> {
    let row:Vec<ImageData> = sqlx::query_as!(
        ImageData, "SELECT * FROM images_data"
    ).fetch_all(&*arc_pool).await.unwrap();
}*/
