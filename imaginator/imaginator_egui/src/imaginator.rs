use data_communicator::buffered::container::DataContainer;
use imaginator_app::mysql_adapter::{MySqlWriter, DB};
use imaginator_types::media::Media;
use sqlx::MySqlPool;
use tokio::{runtime::Handle, task};
use uuid::Uuid;

use crate::apps::{controls::Controls, media_grid::MediaGrid};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Anchor {
    Controls,
    MediaGrid,
}
pub struct Apps {
    anchor: Anchor,
    controls: Controls,
    media_grid: MediaGrid,
}

impl Apps {
    pub fn new(media_cont: &mut DataContainer<Uuid, Media, MySqlWriter>) -> Self {
        Self {
            anchor: Anchor::MediaGrid,
            controls: Controls::new(media_cont.communicator()),
            media_grid: MediaGrid::new(media_cont.communicator()),
        }
    }
}

pub struct Imaginator {
    db: DB,
    apps: Apps,
}

impl eframe::App for Imaginator {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.db.state_update();

        egui::TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                self.bar_contents(ui, frame);
            });
        });
        self.show_selected_app(ctx, frame);
    }
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
        let apps = Apps::new(&mut db.media);
        Self { db, apps }
    }

    fn apps_iter_mut(&mut self) -> impl Iterator<Item = (&str, Anchor, &mut dyn eframe::App)> {
        let vec = vec![
            (
                "Controls",
                Anchor::Controls,
                &mut self.apps.controls as &mut dyn eframe::App,
            ),
            (
                "Media Grid",
                Anchor::MediaGrid,
                &mut self.apps.media_grid as &mut dyn eframe::App,
            ),
        ];

        vec.into_iter()
    }

    fn bar_contents(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::widgets::global_dark_light_mode_switch(ui);

        ui.separator();

        let mut selected_anchor = self.apps.anchor;
        for (name, anchor, _) in self.apps_iter_mut() {
            if ui
                .selectable_label(selected_anchor == anchor, name)
                .clicked()
            {
                selected_anchor = anchor;
            }
        }
        self.apps.anchor = selected_anchor;
    }

    fn show_selected_app(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let selected_anchor = self.apps.anchor;
        for (_, anchor, app) in self.apps_iter_mut() {
            if anchor == selected_anchor || ctx.memory(egui::Memory::everything_is_visible) {
                app.update(ctx, frame);
            }
        }
    }
}


