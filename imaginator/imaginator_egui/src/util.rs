use egui::{ImageSource, Widget};
use egui_extras::{Size, StripBuilder};
use imaginator_app::get_image_path;
use imaginator_types::media::Media;

pub enum WindowSize {
    Small,
    Medium,
    Large,
}

impl WindowSize {
    pub fn from_num(num: usize) -> Self {
        match num {
            0..=699 => Self::Small,
            700..=1199 => Self::Medium,
            _ => Self::Large,
        }
    }
}

pub fn window_size(ui: &egui::Ui) -> WindowSize {
    WindowSize::from_num(ui.available_width().floor() as usize)
}

pub trait GriddableItem {
    fn items_per_row(window_size: &WindowSize) -> usize;
    fn row_height() -> usize;
}

pub fn create_grid<T>(ui: &mut egui::Ui, items: Vec<T>)
where
    T: Widget + GriddableItem + Clone,
{
    let items_per_row = T::items_per_row(&window_size(ui));
    StripBuilder::new(ui)
        .sizes(
            Size::initial(T::row_height() as f32),
            (items.len() as f32 / items_per_row as f32).ceil() as usize,
        )
        .vertical(|mut strip| {
            let chunks = items
                .chunks(items_per_row)
                .map(|chunk| chunk.to_vec())
                .collect::<Vec<_>>();
            for item_chunk in chunks {
                strip.strip(|builder| {
                    builder
                        .sizes(Size::remainder(), items_per_row)
                        .horizontal(|mut strip| {
                            for item in item_chunk {
                                strip.cell(|ui| {
                                    ui.add(item);
                                })
                            }
                        });
                });
            }
        });
}

pub fn media_file_link(media: &Media) -> ImageSource {
    ImageSource::Uri(format!("file://{}", get_image_path(media)).into())
}
