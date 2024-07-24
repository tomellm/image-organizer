#![feature(iter_array_chunks)]

use eframe::NativeOptions;
use egui::ViewportBuilder;
use imaginator::Imaginator;

mod imaginator;
mod util;
mod components;
mod apps;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().expect("dotenv could not load the envirnoment variables.");

    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_drag_and_drop(true),
        ..Default::default()
    };
    let imaginator = Imaginator::new().await;
    let _ = eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(imaginator))
        }),
    );
}

