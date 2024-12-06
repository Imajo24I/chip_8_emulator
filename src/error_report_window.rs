use crate::events::Event;
use crate::utils::button;
use anyhow::Error;
use eframe::egui::Ui;

pub struct ErrorReportWindow {
    error: Error,
}

impl ErrorReportWindow {
    pub fn new(error: Error) -> Self {
        Self { error }
    }

    pub fn update(&mut self, ui: &mut Ui) -> Option<Event> {
        let mut clicked = false;

        ui.vertical_centered(|ui| {
            ui.add_space(20f32);
            ui.end_row();

            ui.label(self.error.to_string());

            if let Some(source) = self.error.source() {
                ui.end_row();
                ui.add_space(20f32);
                ui.label(source.to_string());
            }

            ui.end_row();
            ui.add_space(20f32);
            ui.separator();
            ui.add_space(10f32);

            clicked = button("Exit Program", ui).clicked();
        });

        if clicked {
            return Some(Event::Exit);
        }

        None
    }
}
