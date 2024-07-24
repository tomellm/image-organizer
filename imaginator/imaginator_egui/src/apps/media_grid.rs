use data_communicator::buffered::{communicator::Communicator, query::QueryType};
use egui::Ui;
use imaginator_types::media::Media;
use uuid::Uuid;

use crate::{components::media::MediaCard, util::create_grid};

pub struct MediaGrid {
    media_comm: Communicator<Uuid, Media>,
    pagination: PaginationControls,
}
impl MediaGrid {
    pub fn new(mut media_comm: Communicator<Uuid, Media>) -> Self {
        media_comm.query(QueryType::predicate(|media: &Media| {
            media.media_type.is_image()
        }));
        media_comm.sort(|a, b| a.datetime_created.cmp(&b.datetime_created));
        Self {
            media_comm,
            pagination: PaginationControls::default(),
        }
    }
}

impl eframe::App for MediaGrid {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.media_comm.state_update();
        egui::CentralPanel::default().show(ctx, |ui| {
            self.pagination.controls(ui, self.media_comm.len());
            egui::ScrollArea::vertical().show(ui, |ui| {
                if !self.media_comm.is_empty() {
                    create_grid(
                        ui,
                        self.pagination
                            .paginate(self.media_comm.data_sorted())
                            .into_iter()
                            .map(MediaCard::from)
                            .collect::<Vec<_>>(),
                    )
                }
            });
        });
    }
}

struct PaginationControls {
    page: usize,
    per_page: usize,
}

impl PaginationControls {
    pub fn controls(&mut self, ui: &mut Ui, num_elements: usize) {
        ui.horizontal(|ui| {
            if ui.button("<").clicked() && self.page > 0 {
                self.page -= 1;
            }
            if ui.button("-10").clicked() && self.per_page > 10 {
                self.per_page -= 10;
            }
            if ui.button("-").clicked() && self.per_page > 1 {
                self.per_page -= 1;
            }
            if ui.button("+").clicked() {
                self.per_page += 1;
            }
            if ui.button("+10").clicked() {
                self.per_page += 10;
            }
            if ui.button(">").clicked() && self.page < num_elements / self.per_page {
                self.page += 1;
            }
        });
    }
    pub fn paginate<'a>(&self, to_paginate: Vec<&'a Media>) -> Vec<&'a Media> {
        if to_paginate.is_empty() {
            return to_paginate;
        }
        to_paginate
            .chunks(self.per_page)
            .nth(self.page)
            .unwrap()
            .to_vec()
    }
}

impl Default for PaginationControls {
    fn default() -> Self {
        Self {
            page: 0,
            per_page: 30,
        }
    }
}
