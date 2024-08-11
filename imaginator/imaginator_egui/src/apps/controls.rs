use std::{borrow::Borrow, fs};

use data_communicator::buffered::{
    communicator::Communicator,
    query::QueryType,
};
use egui::{Grid, Spinner, Ui};
use egui_light_states::{
    promise_await::{CreatePromiseAwait, DoneResponse},
    UiStates,
};
use imaginator_app::utils::lazy_async_promise::ChainLazyAsyncPromise;
use imaginator_types::media::Media;
use lazy_async_promise::{ImmediateValuePromise, ImmediateValueState};
use rfd::AsyncFileDialog;
use sysinfo::Disks;
use tracing::warn;
use ubyte::ToByteUnit;
use uuid::Uuid;

pub struct Controls {
    device_selector: DeviceSelector,
    media_comm: Communicator<Uuid, Media>,
    creating_thumbnails: Option<ImmediateValuePromise<()>>,
    small_state: UiStates,
}

impl Controls {
    pub fn new(media_comm: Communicator<Uuid, Media>) -> Self {
        media_comm.query(QueryType::All);
        Self {
            device_selector: DeviceSelector::default(),
            media_comm,
            creating_thumbnails: None,
            small_state: UiStates::default(),
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
                ui.horizontal(|ui| {
                    if ui.button("import the selcted directory").clicked() {
                        let _ = imaginator_app::scan_path_and_save(
                            self.device_selector.selected_dir(),
                            &mut self.media_comm,
                        );
                    }
                    if ui.button("delete all").clicked() {
                        self.media_comm.delete_many(self.media_comm.keys_cloned());
                    }
                    self.small_state.promise_await("load test images")
                        .init_ui(|ui, set_promise| {
                            if ui.button("load test dir").clicked() {
                                let (promise, _) = imaginator_app::scan_path_and_save(
                                    String::from("/Users/tomellm/Documents/coding-projects/image-organizer/working_files/original"),
                                    &mut self.media_comm,
                                );
                                set_promise(promise.then(self.media_comm.query_action(QueryType::All)));
                            }
                        })
                        .waiting_ui(|ui| {
                            ui.add(Spinner::new());
                        })
                        .done_ui(|ui, state| {
                            ui.label(format!("{}", match state {
                                ImmediateValueState::Empty => "empty",
                                ImmediateValueState::Success(_) => "success",
                                ImmediateValueState::Error(_) => "error",
                                ImmediateValueState::Updating => unreachable!()
                            }));
                            if ui.button("reset").clicked() {
                                DoneResponse::Clear
                            } else {
                                DoneResponse::KeepShowing
                            }
                        })
                        .show(ui);
                });
                ui.separator();

                self.small_state.promise_await("creating thumbnail")
                    .init_ui(|ui, set_promise| {
                        if ui.button("create thumbnails").clicked() {
                            let media = self.media_comm.data_cloned();
                            set_promise(ImmediateValuePromise::new(async move {
                                Ok(imaginator_app::create_thumbnails(media).await)
                            }));
                        } else if ui.button("create missing thumbnails").clicked() {
                            let media = self.media_comm.data_cloned();
                            set_promise(ImmediateValuePromise::new(async move {
                                Ok(imaginator_app::create_missing_thumbnails(media).await)
                            }));
                        }
                    })
                    .waiting_ui(|ui| {
                        ui.add(Spinner::new());
                    })
                    .done_ui(|ui, state| {
                        ui.label(format!("{}", match state {
                            ImmediateValueState::Empty => "empty",
                            ImmediateValueState::Success(_) => "success",
                            ImmediateValueState::Error(_) => "error",
                            ImmediateValueState::Updating => unreachable!()
                        }));
                        if ui.button("reset").clicked() {
                            DoneResponse::Clear
                        } else {
                            DoneResponse::KeepShowing
                        }
                    })
                    .show(ui);
                

                if let None = self.creating_thumbnails {
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
            available_space: value.available_space().bytes().to_string(),
            total_space: value.total_space().bytes().to_string(),
        }
    }
}

struct DeviceSelector {
    disks: Vec<Disk>,
    selected_directory: String,
    selected_directory_len: usize,
    selecting_directory: Option<ImmediateValuePromise<Option<String>>>,
}

impl Default for DeviceSelector {
    fn default() -> Self {
        Self {
            selected_directory: String::default(),
            selected_directory_len: 0,
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
                    path.as_ref().map(|selected_directory| {
                        self.selected_directory_len =
                            fs::read_dir(selected_directory).unwrap().count();
                        self.selected_directory = selected_directory.to_owned();
                    });
                    self.selecting_directory = None;
                }
                ImmediateValueState::Error(_) => {
                    warn!("While polling for the result of the directory selection an error occurerd.");
                    self.selecting_directory = None;
                }
                ImmediateValueState::Empty => self.selecting_directory = None,
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
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label(format!("selcted directory: {}", self.selected_directory));
                    ui.label(format!("files in dir: {}", self.selected_directory_len));
                });
            });
        });
    }
    pub fn selected_dir(&self) -> String {
        self.selected_directory.to_owned()
    }
    pub fn select_directory(&mut self, mount_point: String) {
        self.selecting_directory = Some(ImmediateValuePromise::new(async move {
            Ok(AsyncFileDialog::new()
                .set_directory(mount_point)
                .pick_folder()
                .await
                .map(|selected_folder| selected_folder.path().to_str().unwrap().to_string()))
        }));
    }
    fn refresh_disks() -> Vec<Disk> {
        Disks::new_with_refreshed_list()
            .into_iter()
            .map(Disk::from)
            .collect()
    }
}
