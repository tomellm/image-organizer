mod heic;
mod jpg;
pub mod mysql_adapter;
mod png;
mod utils;

use std::{fs, path::Path};

use data_communicator::buffered::communicator::Communicator;
use futures::future::join_all;
use imaginator_importer::{errors::MediaReadErr, scan_path, ReadMediaDirectory};
use imaginator_types::{
    media::Media,
    mediatypes::{ImageType, MediaType},
};
use magick_rust::MagickWand;
use tracing::{debug, info, trace};
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
        .filter(|m| !has_thumbnail(m) && m.media_type.is_image())
        .collect::<Vec<_>>();
    info!(
        "Creating thumbnails for {} medias which are missing one of a totoal of {} medias",
        medias.len(),
        original_len
    );
    create_thumbnails(medias).await
}

pub async fn create_thumbnails(medias: Vec<Media>) {
    const CHUNK_SIZE: usize = 10;
    let medias_len = medias.len();
    let _ = join_all(medias.windows(CHUNK_SIZE).map(|chunk| {
        let medias = chunk.to_owned();
        tokio::spawn(async move {
            create_thumbnail_chunk_magik(medias);
            debug!("Finished chunk of {CHUNK_SIZE} thumbnails");
        })
    }))
    .await;

    info!("Finished creating thumbnails for {} medias", medias_len);
}

#[allow(dead_code)]
fn create_thumbnail_chunk_image(medias: Vec<Media>) {
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
}

#[allow(dead_code)]
fn create_thumbnail_chunk_magik(medias: Vec<Media>) {
    for media in medias {
        if !media.media_type.is_image() {
            continue;
        }
        let wand = MagickWand::new();
        wand.read_image(&images_dir_with_file(media.current_name.as_str()))
            .unwrap();
        wand.fit(200, 200);
        match wand.write_image_blob("jpeg") {
            Ok(bytes) => {
                fs::write(&user_files_with_file(&thumbnail_filename(&media)), bytes)
                    .expect("write failed");
            }
            Err(err) => println!("error: {}", err),
        }
    }
}

pub fn get_image_path(media: &Media) -> String {
    let user_data_path = user_files_with_file(&thumbnail_filename(media));
    if Path::new(&user_data_path).exists() {
        return user_data_path;
    }
    images_dir_with_file(media.current_name.as_str())
}

pub fn thumbnail_path(media: &Media) -> Option<String> {
    let user_data_path = user_files_with_file(&thumbnail_filename(media));
    if Path::new(&user_data_path).exists() {
        return Some(user_data_path);
    }
    None
}

pub fn thumbnail_filename(media: &Media) -> String {
    if media.media_type == MediaType::Image(ImageType::PNG) {
        format!("{}.png", media.uuid)
    } else {
        format!("{}.jpeg", media.uuid)
    }
}
