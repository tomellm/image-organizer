use eframe::NativeOptions;
use egui::ViewportBuilder;
use imaginator::Imaginator;

mod imaginator;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_drag_and_drop(true),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<Imaginator>::default()),
    );
}

