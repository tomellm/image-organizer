use egui::{Direction, Image, Layout, Widget};
use imaginator_types::media::Media;

use crate::util::{thumbnail_file_link, GriddableItem};

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
            ui.with_layout(
                Layout::centered_and_justified(Direction::LeftToRight),
                |ui| {
                    match thumbnail_file_link(self.media) {
                        Some(link) => ui.add(Image::new(link)),
                        None => ui.label("thumbnail not found"),
                    }
                },
            );
            ui.label(self.media.original_name.clone());
            ui.label(format!("{}", self.media.uuid));
        })
        .response
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
