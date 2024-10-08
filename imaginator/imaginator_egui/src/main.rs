#![feature(iter_array_chunks)]

use eframe::NativeOptions;
use egui::ViewportBuilder;
use imaginator::Imaginator;

mod apps;
mod components;
mod imaginator;
mod util;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info,imaginator_app=trace,data_communicator=debug")
        .init();
    dotenv::dotenv().expect("dotenv could not load the envirnoment variables.");
    magick_rust::magick_wand_genesis();

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
