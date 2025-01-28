use crate::emulator_app::Event;
use anyhow::Error;
use eframe::egui::Ui;

pub struct ErrorReportScreen {
    error: Error,
}

impl ErrorReportScreen {
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

            clicked = ui.button("Exit Program").clicked();
        });

        if clicked {
            return Some(Event::Exit);
        }

        None
    }
}
