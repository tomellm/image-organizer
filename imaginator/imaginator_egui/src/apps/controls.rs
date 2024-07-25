use data_communicator::buffered::{communicator::Communicator, query::QueryType};
use egui::Widget;
use imaginator_types::media::Media;
use lazy_async_promise::{ImmediateValuePromise, ImmediateValueState};
use sysinfo::Disks;
use uuid::Uuid;

pub struct Controls {
    device_selector: DeviceSelector,
    media_comm: Communicator<Uuid, Media>,
    creating_thumbnails: Option<ImmediateValuePromise<()>>,
}

impl Controls {
    pub fn new(media_comm: Communicator<Uuid, Media>) -> Self {
        media_comm.query(QueryType::predicate(|_media: &Media| true));
        Self {
            device_selector: DeviceSelector::default(),
            media_comm,
            creating_thumbnails: None,
        }
    }
    pub fn state_update(&mut self) {
        self.media_comm.state_update();
        self.device_selector.state_update();
        if let Some(ref mut promise) = self.creating_thumbnails {
            if !matches!(promise.poll_state(), ImmediateValueState::Updating) {
                self.creating_thumbnails = None;
            }
        }
    }
}

impl eframe::App for Controls {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.state_update();
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.device_selector.ui(ui);
                /*ui.add(egui::TextEdit::singleline(&mut self.selected_dir));
                if ui.button("load stuff").clicked() {
                    self.parse_dir();
                }*/
                if let None = self.creating_thumbnails {
                    if ui.button("create thumbnails").clicked() {
                        let media = self.media_comm.data_cloned();
                        self.creating_thumbnails = Some(ImmediateValuePromise::new(async move {
                            Ok(imaginator_app::create_thumbnails(media).await)
                        }));
                    }
                    if ui.button("create missing thumbnails").clicked() {
                        let media = self.media_comm.data_cloned();
                        self.creating_thumbnails = Some(ImmediateValuePromise::new(async move {
                            Ok(imaginator_app::create_missing_thumbnails(media).await)
                        }));
                    }
                }
                ui.label(format!("len {}", self.media_comm.data().len()));
            });
        });
    }
}

struct DeviceSelector {
    pub selected_path: String,
    pub decive_selecting: Option<ImmediateValuePromise<String>>,
    pub selected_device: String,
    disks: Disks,
}

impl Default for DeviceSelector {
    fn default() -> Self {
        Self {
            selected_path: String::default(),
            decive_selecting: None,
            selected_device: String::default(),
            disks: Disks::new_with_refreshed_list()
        }
    }
}

impl DeviceSelector {
    pub fn state_update(&mut self) {
        if let Some(ref mut promise) = self.decive_selecting {
            if !matches!(promise.poll_state(), ImmediateValueState::Updating) {
                self.decive_selecting = None;
            }
        }
    }
    pub fn ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            for disk in &self.disks {
                ui.label(disk.name().to_str().unwrap());
            }
        }).response
    }
    /*pub fn parse_dir(&mut self) {
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
    }*/
}
