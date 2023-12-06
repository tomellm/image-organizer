use std::sync::Arc;

use sqlx::{MySql, Pool};
use types::database::ImageData;
use uuid::Uuid;

pub async fn save_one(
    pool: Arc<Pool<MySql>>,
    image: ImageData
) -> Result<(), ()> {
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

pub async fn get_all(
    pool: Arc<Pool<MySql>>
) -> Result<Vec<ImageData>, ()> {
    let out = sqlx::query_as!(ImageData, "select * from images_data")
        .fetch_all(&*pool)
        .await
        .map_err(|err| eprintln!("ERROR: get_images failed to execute query. {err}"))?;
    Ok(out)
}

pub async fn get_one(
    pool: Arc<Pool<MySql>>,
    uuid: Uuid
) -> Result<ImageData, ()> {
    let out = sqlx::query_as!(
        ImageData,
        "select * from images_data where uuid = ?",
        uuid.simple().to_string()
    )
    .fetch_one(&*pool)
    .await
    .map_err(|err| eprintln!("ERROR: get_image failed to execute query {err}"))?;

    Ok(out)
}

