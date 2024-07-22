use image::{imageops::FilterType, ImageBuffer, Rgb, RgbImage};
use imaginator_types::{
    media::Media,
    mediatypes::{ImageType, MediaType},
};
use libheif_rs::{ColorSpace, HeifContext, LibHeif, Planes, RgbChroma};
use tracing::error;

use crate::utils::{images_dir_with_file, scale_down_to_max, user_files_with_file};

pub fn parse_heic(heic_media: &Media) {
    let Media {
        uuid,
        current_name,
        media_type,
        ..
    } = heic_media;
    assert_eq!(*media_type, MediaType::Image(ImageType::HEIC));

    let lib_heic = LibHeif::new();
    let file_path = images_dir_with_file(&current_name);
    let Ok(ctx) = HeifContext::read_from_file(&file_path) else {
        error!("The following file path {} threw this error.", file_path);
        return;
    };
    let handle = ctx.primary_image_handle().unwrap();
    let image = lib_heic
        .decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)
        .unwrap();
    let Planes {
        interleaved: Some(inter),
        y: None,
        cb: None,
        cr: None,
        r: None,
        g: None,
        b: None,
        a: None,
    } = image.planes()
    else {
        error!("The planes of the image as not the expected ones for Media: {uuid}");
        let planes = image.planes();
        dbg!(
            planes.y.is_some(),
            planes.cb.is_some(),
            planes.cr.is_some(),
            planes.r.is_some(),
            planes.g.is_some(),
            planes.b.is_some(),
            planes.a.is_some(),
            planes.interleaved.is_some()
        );
        return;
    };

    if inter.bits_per_pixel * 3 != inter.storage_bits_per_pixel {
        error!(
            "The bits per pixel were not a thrid of the total storage bits. {} {}",
            inter.bits_per_pixel, inter.storage_bits_per_pixel
        );
        return;
    }


    /*for (x, y, pixel) in image_buff.enumerate_pixels_mut() {
        let pos = (image.width() * y + x) as usize;
        *pixel = image::Rgb([r.data[pos], g.data[pos], b.data[pos]]);
    }*/
    let buffer = ImageBuffer::<Rgb<u8>, &[u8]>::from_raw(handle.width(), handle.height(), inter.data).unwrap();
    let (new_width, new_height) = scale_down_to_max(handle.width(), handle.height());
    let buffer = image::imageops::resize(&buffer, new_width, new_height, FilterType::Nearest);
    image::save_buffer_with_format(
        user_files_with_file(format!("{uuid}.jpeg").as_str()),
        &buffer,
        new_width,
        new_height,
        image::ExtendedColorType::Rgb8,
        image::ImageFormat::Jpeg,
    )
    .unwrap();
}
