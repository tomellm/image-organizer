use std::path::Path;

use image::imageops::FilterType;
use imaginator_types::{
    media::Media,
    mediatypes::{ImageType, MediaType},
};

use crate::{
    thumbnail_filename,
    utils::{images_dir_with_file, scale_down_to_max, user_files_with_file},
};

pub fn jpg_thumbnail(jpg_media: &Media) {
    let Media {
        current_name,
        media_type,
        ..
    } = jpg_media;
    assert!(
        *media_type == MediaType::Image(ImageType::JPG)
            || *media_type == MediaType::Image(ImageType::JPEG)
    );
    let dyn_image = image::open(Path::new(&images_dir_with_file(current_name.as_str()))).unwrap();
    let (new_width, new_height) = scale_down_to_max(dyn_image.width(), dyn_image.height());
    let dyn_image = dyn_image.resize(new_width, new_height, FilterType::Gaussian);
    let _ = dyn_image
        .save_with_format(
            user_files_with_file(&thumbnail_filename(jpg_media)),
            image::ImageFormat::Jpeg,
        )
        .unwrap();
}
