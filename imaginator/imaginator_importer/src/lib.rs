pub mod adapters;
pub mod errors;

use std::sync::Arc;

use adapters::filesystem_adapter::get_media_with_xmp;
use data_communicator::buffered::communicator::Communicator;
use errors::MediaReadErr;
use imaginator_types::media::Media;
use postgres_adapter::save_new_media;
use sqlx::{MySql, Pool};
use uuid::Uuid;

pub struct ReadMediaDirectory {
    media: Vec<Media>,
    errors: Vec<MediaReadErr>,
}

pub fn scan_path_and_save(
    path: String,
    communicator: &mut Communicator<Uuid, Media>,
) -> Vec<MediaReadErr> {
    let media = get_media_with_xmp(&path).unwrap();
    communicator.update_many(media.media);
    media.errors
}
