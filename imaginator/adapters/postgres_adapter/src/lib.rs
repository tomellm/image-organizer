#![allow(dead_code)]

mod media_data;
mod meta_data;
mod types;
mod util;
mod xmp_data;
use futures::try_join;
use uuid::Uuid;
use std::sync::Arc;

use imaginator_types::media::Media;
use sqlx::{MySql, Pool};
use types::{FromDBUuid, MediaData, MediaUnwrapped};

pub async fn save_new_media(pool: Arc<Pool<MySql>>, media: Vec<Media>) -> Result<(), ()> {
    let db_media = media
        .into_iter()
        .map(MediaUnwrapped::from)
        .collect::<Vec<_>>();

    let (media_data, meta_data, xmp_data) = db_media.into_iter().fold(
        (vec![], vec![], vec![]),
        |(mut med, mut meta, mut xmp), MediaUnwrapped(new_med, new_meta, new_xmp)| {
            med.push(new_med);
            meta.extend(new_meta);
            xmp.extend(new_xmp);
            (med, meta, xmp)
        },
    );

    let media_fut = media_data::save_many(pool.clone(), media_data);
    let meta_fut = meta_data::save_many(pool.clone(), meta_data);
    let xmp_fut = xmp_data::save_many(pool, xmp_data);

    match try_join!(media_fut, meta_fut, xmp_fut) {
        Ok(((), (), ())) => Ok(()),
        _ => Err(()),
    }
}

pub async fn get_all_media(pool: Arc<Pool<MySql>>) -> Result<Vec<Media>, ()> {
    build_medias(pool.clone(), media_data::get_all(pool).await?).await
}

pub async fn delete_all(pool: Arc<Pool<MySql>>) -> Result<(), ()> {
    media_data::delete_all(pool).await
}

async fn build_medias(pool: Arc<Pool<MySql>>, images: Vec<MediaData>) -> Result<Vec<Media>, ()> {
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
            let uuid = Uuid::from_db(&id.uuid).unwrap();
            let xmp = xmp_data.remove(&uuid).unwrap_or(vec![]);
            let meta = meta_data.remove(&uuid).unwrap_or(vec![]);
            Media::from(MediaUnwrapped(id, meta, xmp))
        })
        .collect())
}
