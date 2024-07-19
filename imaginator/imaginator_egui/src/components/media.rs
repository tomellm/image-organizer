use egui::{Image, ImageSource, Widget};
use imaginator_types::{media::Media, mediatypes::{ImageType, MediaType}};

use crate::util::GriddableItem;

#[derive(Clone)]
pub struct MediaCard<'a> {
    media: &'a Media,
}

impl<'a> From<&'a Media> for MediaCard<'a> {
    fn from(media: &'a Media) -> Self {
        Self { media }
    }
}

impl<'a> Widget for MediaCard<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            if Some(ImageType::HEIC) != self.media.media_type.image() && self.media.media_type.is_image() {
                let uri = format!(
                    "file://../../working_files/original/{}",
                    self.media.original_name
                    );
                ui.add(Image::new(ImageSource::Uri(uri.into())));
            }
            ui.label(self.media.original_name.clone());
            ui.label(format!("{:?}", self.media.media_type));
        }).response
    }
}

impl<'a> GriddableItem for MediaCard<'a> {
    fn items_per_row(window_size: &crate::util::WindowSize) -> usize {
        match window_size {
            crate::util::WindowSize::Large => 8,
            crate::util::WindowSize::Medium => 5,
            crate::util::WindowSize::Small => 2,
        }
    }
    fn row_height() -> usize {
        150
    }
}
