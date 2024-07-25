use std::path::Path;

use data_communicator::buffered::{communicator::Communicator, query::QueryType};
use egui::{Grid, TextBuffer, Ui, Widget};
use egui_extras::{Column, Table, TableBuilder};
use imaginator_types::media::Media;
use lazy_async_promise::{ImmediateValuePromise, ImmediateValueState};
use rfd::AsyncFileDialog;
use sysinfo::Disks;
use tracing::warn;
use ubyte::{ByteUnit, ToByteUnit};
use uuid::Uuid;

pub struct Controls {
    device_selector: DeviceSelector,
    media_comm: Communicator<Uuid, Media>,
    creating_thumbnails: Option<ImmediateValuePromise<()>>,
}
    //let result_one = imaginator_app::scan_path_and_save(dir, &mut self.media_comm);

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

#[derive(Clone)]
struct Disk {
    name: String,
    file_system: String,
    is_removable: String,
    kind: String,
    mount_point: String,
    available_space: String,
    total_space: String,
}

impl From<&sysinfo::Disk> for Disk {
    fn from(value: &sysinfo::Disk) -> Self {
        Self { 
            name: value.name().to_str().unwrap().to_string(),
            file_system: value.file_system().to_str().unwrap().to_string(),
            is_removable: value.is_removable().to_string(),
            kind: value.kind().to_string(),
            mount_point: value.mount_point().to_str().unwrap().to_string(),
            available_space: ByteUnit::Byte(value.available_space()).gigabytes().to_string(),
            total_space: ByteUnit::Byte(value.total_space()).gigabytes().to_string()
        }
    }
}

struct DeviceSelector {
    disks: Vec<Disk>,
    pub selected_directory: String,
    pub selecting_directory: Option<ImmediateValuePromise<String>>,
}

impl Default for DeviceSelector {
    fn default() -> Self {
        Self {
            selected_directory: String::default(),
            selecting_directory: None,
            disks: Self::refresh_disks(),
        }
    }
}

impl DeviceSelector {
    pub fn state_update(&mut self) {
        if let Some(ref mut promise) = self.selecting_directory {
            match promise.poll_state() {
                ImmediateValueState::Updating => (),
                ImmediateValueState::Success(path) => {
                    self.selected_directory = path.to_owned();
                    self.selecting_directory = None;
                },
                ImmediateValueState::Error(_) => {
                    warn!("While polling for the result of the directory selection an error occurerd.");
                    self.selecting_directory = None;
                },
                ImmediateValueState::Empty => self.selecting_directory = None
            }
        }
    }
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                Grid::new("disks_table").show(ui, |ui: &mut Ui| {
                    if ui.button("refresh").clicked() {
                        self.disks = Self::refresh_disks();
                    }
                    ui.label("name");
                    ui.label("file_system");
                    ui.label("is_removable");
                    ui.label("kind");
                    ui.label("mount_point");
                    ui.label("available_space");
                    ui.label("total_space");
                    ui.label("select folder");
                    ui.end_row();
                    for disk in self.disks.iter().cloned().collect::<Vec<_>>() {
                        ui.label("");
                        ui.label(disk.name);
                        ui.label(disk.file_system);
                        ui.label(disk.is_removable);
                        ui.label(disk.kind);
                        ui.label(disk.mount_point.clone());
                        ui.label(disk.available_space);
                        ui.label(disk.total_space);
                        if ui.button("select").clicked() {
                            self.select_directory(disk.mount_point);
                        }
                        ui.end_row();
                    }
                });
            });
            ui.horizontal(|ui| {
                ui.label(format!("selcted directory: {}", self.selected_directory));
            });
        });
    }
    pub fn select_directory(&mut self, mount_point: String) {
        self.selecting_directory = Some(ImmediateValuePromise::new(async move {
            Ok(AsyncFileDialog::new()
                .set_directory(mount_point)
                .pick_folder()
                .await
                .unwrap()
                .path()
                .to_str()
                .unwrap()
                .to_string())
        }));
    }
    fn refresh_disks() -> Vec<Disk> {
        Disks::new_with_refreshed_list().into_iter().map(Disk::from).collect()
    }
}
