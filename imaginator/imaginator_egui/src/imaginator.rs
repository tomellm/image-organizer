use data_communicator::buffered::{communicator::Communicator, query::QueryType};
use egui::{Image, ImageSource, Rect, Ui};
use egui_extras::{install_image_loaders, StripBuilder};
use imaginator_importer::adapters::mysql_adapter::DB;
use imaginator_types::{
    media::Media,
    mediatypes::{ImageType, MediaType},
};
use sqlx::MySqlPool;
use tokio::{runtime::Handle, task};
use tracing::info;
use uuid::Uuid;

use crate::{components::media::MediaCard, util::create_grid};

pub struct Imaginator {
    selected_dir: String,
    db: DB,
    media_comm: Communicator<Uuid, Media>,
}

impl Imaginator {
    pub async fn new() -> Self {
        let conn_str = "mysql://root:password@localhost:5432/imaginator";

        let conn = task::block_in_place(|| {
            Handle::current()
                .block_on(MySqlPool::connect(&conn_str))
                .unwrap()
        });

        let mut db = DB::init(conn).await;
        let media_comm = db.media.communicator();
        media_comm.query(QueryType::predicate(|media: &Media| {
            Some(ImageType::HEIC) != media.media_type.image() && media.media_type.is_image()
        }));

        Self {
            selected_dir: String::new(),
            db,
            media_comm,
        }
    }
}

impl eframe::App for Imaginator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.db.state_update();
        self.media_comm.state_update();

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(egui::TextEdit::singleline(&mut self.selected_dir));
                if ui.button("load stuff").clicked() {
                    self.parse_dir();
                }
                ui.label(format!("len {}", self.media_comm.data.data.len()));

                create_grid(
                    ui,
                    self.media_comm
                        .data
                        .data
                        .iter()
                        .map(|(_, media)| MediaCard::from(media))
                        .collect::<Vec<_>>(),
                )
            });
        });
    }
}

impl Imaginator {
    pub fn delete_all(&mut self) {
        self.media_comm.delete_many(
            self.media_comm
                .data
                .data
                .keys()
                .map(|k| *k)
                .collect::<Vec<_>>(),
        );
    }
    pub fn parse_dir(&mut self) {
        let dir = self.selected_dir.clone();
        let result_one = imaginator_importer::scan_path_and_save(dir, &mut self.media_comm);
        if !result_one.is_empty() {
            tracing::event!(
                tracing::Level::ERROR,
                "Saving images caused {} errors",
                result_one.len()
            );
        }
        self.media_comm.query(QueryType::predicate(|_| true));
    }
}
