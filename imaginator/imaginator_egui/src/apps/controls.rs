use data_communicator::buffered::{communicator::Communicator, query::QueryType};
use imaginator_types::media::Media;
use lazy_async_promise::{ImmediateValuePromise, ImmediateValueState};
use uuid::Uuid;

pub struct Controls {
    selected_dir: String,
    media_comm: Communicator<Uuid, Media>,
    creating_thumbnails: Option<ImmediateValuePromise<()>>,
}

impl Controls {
    pub fn new(media_comm: Communicator<Uuid, Media>) -> Self {
        media_comm.query(QueryType::predicate(|_| true));
        Self {
            selected_dir: String::new(),
            media_comm,
            creating_thumbnails: None,
        }
    }
    pub fn state_update(&mut self) {
        self.media_comm.state_update();
        if let Some(ref mut promise) = self.creating_thumbnails {
            let state = promise.poll_state();
            if matches!(state, ImmediateValueState::Updating) {
                self.creating_thumbnails = None;
            }
        }
    }
    pub fn parse_dir(&mut self) {
        let dir = self.selected_dir.clone();
        let result_one = imaginator_app::scan_path_and_save(dir, &mut self.media_comm);
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

impl eframe::App for Controls {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.state_update();
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(egui::TextEdit::singleline(&mut self.selected_dir));
                if ui.button("load stuff").clicked() {
                    self.parse_dir();
                }
                if ui.button("create thumbnails").clicked() {
                    let media = self.media_comm.data_cloned();
                    self.creating_thumbnails = Some(ImmediateValuePromise::new(async move {
                        Ok(imaginator_app::create_thumbnails(media).await)
                    }));
                }
                ui.label(format!("len {}", self.media_comm.data().len()));
            });
        });
        
    }
}
