use crate::errors::error::Error;
use crate::events::Event;
use crate::utils;
use eframe::egui::{FontId, RichText, Ui};

pub struct ErrorReportWindow {
    error: Error,
}

impl ErrorReportWindow {
    pub fn new(error: Error) -> Self {
        Self {
            error,
        }
    }

    pub fn update(&mut self, ui: &mut Ui) -> Option<Event> {
        let mut clicked = false;

        ui.vertical_centered(|ui| {
            ui.add_space(20f32);

            ui.heading(RichText::new("Error executing Chip 8 Emulator")
                .font(FontId::proportional(40f32)));

            ui.add_space(30f32);
            ui.separator();
            ui.add_space(60f32);

            ui.end_row();

            //TODO: Add button to view error cause
            ui.label(self.error.to_string());

            ui.end_row();
            ui.add_space(60f32);
            ui.separator();
            ui.add_space(20f32);

            clicked = utils::button("Exit Program", ui).clicked();
        });

        if clicked {
            return Some(Event::Exit);
        }

        None
    }
}
