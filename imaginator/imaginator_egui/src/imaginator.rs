use std::sync::Arc;

use imaginator_types::media::Media;
use lazy_async_promise::{ImmediateValuePromise, ImmediateValueState};
use sqlx::{MySql, MySqlPool, Pool};
use tokio::{runtime::Handle, task};

pub struct Imaginator {
    selected_dir: String,
    parsed_media: Option<ImmediateValuePromise<Vec<Media>>>,
    pool: Arc<Pool<MySql>>,
    images: Vec<String>,
    delete_all_task: Option<ImmediateValuePromise<()>>
}

impl Default for Imaginator {
    fn default() -> Self {
        let conn_str = "mysql://root:password@localhost:5432/imaginator";

        let conn = task::block_in_place(|| {
            Handle::current()
                .block_on(MySqlPool::connect(&conn_str))
                .unwrap()
        });

        Self {
            pool: Arc::new(conn),
            selected_dir: String::new(),
            parsed_media: None,
            images: vec![],
            delete_all_task: None
        }
    }
}

impl eframe::App for Imaginator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(ref mut media) = self.parsed_media {
            match media.poll_state() {
                ImmediateValueState::Success(new_media) => {
                    let to_add = new_media.into_iter()
                        .filter(|e| e.extension.to_uppercase().eq("JPG"))
                        .map(|e| e.current_name.clone())
                        .take(10)
                        .collect::<Vec<_>>();
                    self.images.extend(to_add);
                },
                _ => ()
            }
        }


        egui::TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(egui::TextEdit::singleline(&mut self.selected_dir));
                if ui.button("load stuff").clicked() {
                    self.parse_dir();
                }
                ui.label(format!("len {}", self.images.len()));
            });
        });
    }
}

impl Imaginator {
    pub fn delete_all(&mut self) {
    }
    pub fn parse_dir(&mut self) {
        let new_pool = self.pool.clone();
        let dir = self.selected_dir.clone();
        self.parsed_media = Some(ImmediateValuePromise::new(async move {
            let result_one = imaginator_importer::scan_path_and_save(new_pool.clone(), dir).await;
            if !result_one.is_empty() {
                tracing::event!(
                    tracing::Level::ERROR,
                    "Saving images cause {} errors",
                    result_one.len()
                );
            }

            Ok(imaginator_app::view_all_media(new_pool).await.unwrap())
        }));
    }
}
