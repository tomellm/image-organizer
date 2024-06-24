pub mod errors;
mod adapters;

use std::sync::Arc;

use adapters::filesystem_adapter::get_media_with_xmp;
use errors::MediaReadErr;
use imaginator_types::media::Media;
use postgres_adapter::save_new_media;
use sqlx::{MySql, Pool};

pub struct ReadMediaDirectory {
    media: Vec<Media>,
    errors: Vec<MediaReadErr>,
}


pub async fn scan_path_and_save(
    pool: Arc<Pool<MySql>>,
    path: String
) -> Vec<MediaReadErr> {
    let media = get_media_with_xmp(&path).unwrap();
    save_new_media(pool, media.media).await.unwrap();
    media.errors
}
