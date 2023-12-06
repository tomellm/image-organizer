mod image_data;
mod meta_data;
mod xmp_data;

use std::sync::Arc;

use sqlx::{MySql, Pool, QueryBuilder};
use types::{database::{ImageData, ImageXmpData, ImageMetaData}, image::Image};
use uuid::Uuid;

pub async fn save_image(
    pool: Arc<Pool<MySql>>,
    image: Image
) -> Result<(), ()> {
    let (image, meta, xmp) = image.to_db();
    

    image_data::save_one(pool.clone(), image).await?;
    meta_data::save_many(pool.clone(), meta).await?;
    xmp_data::save_many(pool.clone(), xmp).await?;

    Ok(())
}

pub async fn get_images(
    pool: Arc<Pool<MySql>>
) -> Result<Vec<Image>, ()> {
    let out = image_data::get_all(pool.clone()).await?;
    let all_uuids = out.iter().map(|i| i.uuid.clone()).collect::<Vec<_>>();

    let mut xmp_data = xmp_data::get_by_str_images(pool.clone(), &all_uuids)
        .await?;
    let mut meta_data = meta_data::get_by_str_images(pool.clone(), &all_uuids)
        .await?;

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

pub async fn get_image(
    pool: Arc<Pool<MySql>>,
    uuid: Uuid
) -> Result<Image, ()> {
    let out = image_data::get_one(pool.clone(), uuid).await?;
    let meta = meta_data::get_by_image(pool.clone(), uuid).await?;
    let xmp = xmp_data::get_by_image(pool.clone(), uuid).await?;
    
    Ok(out.to_struct(meta, xmp))
}
