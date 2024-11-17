use crate::errors::error::Error;
use crate::events::Event;
use crate::utils::button;
use eframe::egui;
use eframe::egui::Ui;

pub struct ErrorReportWindow {
    error: Error,
    view_cause: bool,
}

impl ErrorReportWindow {
    pub fn new(error: Error) -> Self {
        Self {
            error,
            view_cause: false,
        }
    }

    pub fn update(&mut self, ui: &mut Ui) -> Option<Event> {
        let mut clicked = false;

        ui.vertical_centered(|ui| {
            ui.add_space(20f32);
            ui.end_row();

            ui.label(&self.error.message);

            self.view_additional_info(ui);

            ui.end_row();
            ui.add_space(20f32);
            ui.separator();
            ui.add_space(10f32);

            clicked = button("Exit Program", ui).clicked();

            ui.end_row();
            ui.add_space(10f32);
        });

        if clicked {
            return Some(Event::Exit);
        }

        None
    }

    fn view_additional_info(&mut self, ui: &mut Ui) {
        ui.end_row();
        ui.add_space(20f32);

        if button("View additional Information", ui).clicked() {
            self.view_cause = true;
        }
        if self.view_cause {
            egui::Window::new("Error Information").default_size([840f32, 530f32]).show(ui.ctx(), |ui| {
                ui.vertical_centered(|ui| {
                    if let Some(info) = &self.error.cause.additional_info {
                        ui.label(info);
                    }

                    if let Some(error) = &self.error.cause.error {
                        ui.end_row();
                        ui.add_space(40f32);

                        ui.label(error.to_string());
                    }

                    ui.end_row();
                    ui.add_space(20f32);

                    if button("Close", ui).clicked() {
                        self.view_cause = false;
                    }
                });
            });
        }
    }
}
