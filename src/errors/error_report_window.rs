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

            self.view_cause(ui);

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

    fn view_cause(&mut self, ui: &mut Ui) {
        ui.end_row();
        ui.add_space(20f32);

        if let Some(cause) = &self.error.cause {
            if button("View cause", ui).clicked() {
                self.view_cause = true;
            }
            if self.view_cause {
                egui::Window::new("Error Cause").default_size([840f32, 530f32]).show(ui.ctx(), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(cause.to_string());

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
}
