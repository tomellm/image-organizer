#![allow(dead_code)]

mod media_data;
mod meta_data;
mod types;
mod util;
mod xmp_data;
use futures::try_join;
use std::sync::Arc;

use imaginator_types::media::Media;
use sqlx::{MySql, Pool};
use types::MediaUnwrapped;

pub async fn save_new_media(pool: Arc<Pool<MySql>>, media: Vec<Media>) -> Result<(), ()>{
    let db_media = media.into_iter().map(MediaUnwrapped::from).collect::<Vec<_>>();

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
        _ => Err(())
    }
}

