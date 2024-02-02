mod image_data;
mod meta_data;
mod xmp_data;
mod util;


use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {

use std::{sync::Arc, time::Instant, collections::HashMap};

use std::collections::hash_map;
use sqlx::{MySql, Pool};
use tokio::try_join;
use tracing::event;
use types::{database::{MediaData, MediaXmpData, MediaMetaData}, image::Media, args::Pagination, metadata::MetaData, xmpdata::XmpData};
use uuid::Uuid;

pub async fn get_media(
    pool: Arc<Pool<MySql>>,
    uuid: Uuid
) -> Result<Media, ()> {
    let out = image_data::get_one(pool.clone(), uuid).await?;
    let meta = meta_data::get_by_media(pool.clone(), uuid).await?;
    let xmp = xmp_data::get_by_media(pool.clone(), uuid).await?;
    
    Ok(out.to_struct(meta, xmp))
}

pub async fn get_many_media(
    pool: Arc<Pool<MySql>>,
    uuids: Vec<Uuid>
) -> Result<Vec<Media>, ()> {
    build_medias(
        pool.clone(),
        image_data::get_many(pool, uuids).await?
    ).await
}

pub async fn get_all_medias(
    pool: Arc<Pool<MySql>>
) -> Result<Vec<Media>, ()> {
    build_medias(
        pool.clone(),
        image_data::get_all(pool).await?
    ).await
}

pub async fn get_all_images(
    pool: Arc<Pool<MySql>>
) -> Result<Vec<Media>, ()> {
    build_medias(
        pool.clone(),
        image_data::get_all_images(pool).await?
    ).await
}

pub async fn get_images_paginated(
    pool: Arc<Pool<MySql>>,
    page: Pagination
) -> Result<Option<Vec<Media>>, ()> {
    let images = image_data::get_images_paginated(pool.clone(), page).await?;

    match images {
        None => Ok(None),
        Some(images) => build_medias(pool, images).await.map(|m| Some(m))
    }
}

pub async fn save_media(
    pool: Arc<Pool<MySql>>,
    image: Media
) -> Result<(), ()> {
    let (image, meta, xmp) = image.to_db();

    image_data::save_one(pool.clone(), image).await?;
    meta_data::save_many(pool.clone(), meta).await?;
    xmp_data::save_many(pool.clone(), xmp).await?;

    Ok(())
}

pub async fn save_medias(
    pool: Arc<Pool<MySql>>,
    images: Vec<Media>
) -> Result<(), ()> {

    let start = Instant::now();
    
    let (img_acc, meta_acc, xmp_acc) = images.into_iter()
        .map(Media::to_db)
        .fold(
            (vec![], vec![], vec![]), 
            |
                (mut img_acc, mut meta_acc, mut xmp_acc): (Vec<MediaData>, Vec<MediaMetaData>,Vec<MediaXmpData>), 
                (img, mut meta, mut xmp)
            |{
                img_acc.push(img);
                meta_acc.append(&mut meta);
                xmp_acc.append(&mut xmp);
                (img_acc, meta_acc, xmp_acc)
            }
        );

    event!(
        tracing::Level::INFO,
        "About to start saving: {} images, {} meta data points {} xmp data points",
        img_acc.len(), meta_acc.len(), xmp_acc.len()
    );
    
    try_join!(
        image_data::save_many(pool.clone(), img_acc),
        meta_data::save_many(pool.clone(), meta_acc),
        xmp_data::save_many(pool.clone(), xmp_acc)
    )?;

    tracing::event!(
        tracing::Level::INFO,
        "Saving all images took: {:?}",
        start.elapsed());

    Ok(())
}



pub async fn get_all_dates(
    pool: Arc<Pool<MySql>>
) -> Result<HashMap<Uuid, (Vec<MetaData>, Vec<XmpData>)>, ()> {
    let meta = meta_data::get_all_dates(pool.clone()).await?;
    let xmp = xmp_data::get_all_dates(pool).await?;

    let mut map: HashMap<Uuid, (Vec<MetaData>, Vec<XmpData>)> = HashMap::new();

    meta.into_iter()
        .for_each(|dp| {
            let uuid = Uuid::parse_str(&dp.media_uuid).unwrap();
            let struct_d = dp.to_struct().unwrap();
            map.entry(uuid)
                .and_modify(|o| o.0.push(struct_d.clone()))
                .or_insert((vec![struct_d], vec![]));
        });

    xmp.into_iter()
        .for_each(|dp| {
            let uuid = Uuid::parse_str(&dp.media_uuid).unwrap();
            let struct_d = dp.to_struct().unwrap();
            map.entry(uuid)
                .and_modify(|o| o.1.push(struct_d.clone()))
                .or_insert((vec![], vec![struct_d]));
        });

    Ok(map)
}


async fn build_medias(
    pool: Arc<Pool<MySql>>,
    images: Vec<MediaData>
) -> Result<Vec<Media>, ()> {
    if images.is_empty() {
        return Ok(vec![]);
    }

    let all_uuids = images.iter().map(|i| i.uuid.clone()).collect::<Vec<_>>();

    let (mut xmp_data, mut meta_data) = try_join!(
        xmp_data::get_by_str_medias(pool.clone(), &all_uuids),
        meta_data::get_by_str_medias(pool.clone(), &all_uuids)
    )?;

    Ok(images
        .into_iter()
        .map(|id| {
            let uuid = Uuid::parse_str(&id.uuid).unwrap();
            let xmp = xmp_data.remove(&uuid).unwrap_or(vec![]);
            let meta = meta_data.remove(&uuid).unwrap_or(vec![]);
            id.to_struct(meta, xmp)
        })
        .collect())
}
}}
