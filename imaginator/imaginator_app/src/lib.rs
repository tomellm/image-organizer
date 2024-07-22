mod heic;
pub mod mysql_adapter;
mod utils;

use std::path::Path;

use data_communicator::buffered::communicator::Communicator;
use futures::future::join_all;
use imaginator_importer::{errors::MediaReadErr, scan_path, ReadMediaDirectory};
use imaginator_types::{
    media::Media,
    mediatypes::{ImageType, MediaType},
};
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

pub async fn create_thumbnails(medias: Vec<Media>) {
    let mut handles = medias
        .windows(10)
        .map(|chunk| {
            let medias = chunk.to_owned();
            tokio::spawn(async move {
                for media in medias {
                    if media.media_type == MediaType::Image(ImageType::HEIC) {
                        heic::parse_heic(&media);
                    }
                }
            })
        })
        .collect::<Vec<_>>();
    join_all(handles.drain(..)).await;
}

pub fn get_image_path(media: &Media) -> String {
    let user_data_path = user_files_with_file(format!("{}.jpeg",media.uuid).as_str());
    if Path::new(&user_data_path).exists() {
        return user_data_path;
    }
    images_dir_with_file(media.current_name.as_str())
}
