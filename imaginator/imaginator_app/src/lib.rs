mod heic;
mod jpg;
pub mod mysql_adapter;
mod png;
mod utils;

use std::path::Path;

use data_communicator::buffered::communicator::Communicator;
use futures::future::join_all;
use imaginator_importer::{errors::MediaReadErr, scan_path, ReadMediaDirectory};
use imaginator_types::{
    media::Media,
    mediatypes::{ImageType, MediaType},
};
use tracing::{info, trace};
use utils::{images_dir_with_file, user_files_with_file};
use uuid::Uuid;

pub fn scan_path_and_save(
    path: String,
    communicator: &mut Communicator<Uuid, Media>,
) -> Vec<MediaReadErr> {
    let ReadMediaDirectory { media, errors } = scan_path(path);
    communicator.update_many(media);
    errors
}

pub fn has_thumbnail(media: &Media) -> bool {
    Path::new(&user_files_with_file(&thumbnail_filename(media))).exists()
}

pub async fn create_missing_thumbnails(medias: Vec<Media>) {
    let original_len = medias.len();
    let medias = medias
        .into_iter()
        .filter(|m| !has_thumbnail(m))
        .collect::<Vec<_>>();
    info!(
        "Creating thumbnails for {} medias which are missing one of a totoal of {} medias",
        medias.len(),
        original_len
    );
    create_thumbnails(medias).await
}

pub async fn create_thumbnails(medias: Vec<Media>) {
    let medias_len = medias.len();
    let mut handles = medias
        .windows(10)
        .map(|chunk| {
            let medias = chunk.to_owned();
            tokio::spawn(async move {
                for media in medias {
                    let MediaType::Image(image_type) = media.media_type else {
                        continue;
                    };
                    match image_type {
                        ImageType::HEIC => heic::heic_thumbnail(&media),
                        ImageType::JPEG | ImageType::JPG => jpg::jpg_thumbnail(&media),
                        ImageType::PNG => png::png_thumbnail(&media),
                    }
                    trace!("Finished creating thumbnail for media {}", media.uuid);
                }
            })
        })
        .collect::<Vec<_>>();
    join_all(handles.drain(..)).await;
    info!("Finished creating thumbnails for {} medias", medias_len);
}

pub fn get_image_path(media: &Media) -> String {
    let user_data_path = user_files_with_file(&thumbnail_filename(media));
    if Path::new(&user_data_path).exists() {
        return user_data_path;
    }
    images_dir_with_file(media.current_name.as_str())
}

pub fn thumbnail_filename(media: &Media) -> String {
    if media.media_type == MediaType::Image(ImageType::PNG) {
        format!("{}.png", media.uuid)
    } else {
        format!("{}.jpeg", media.uuid)
    }
}
