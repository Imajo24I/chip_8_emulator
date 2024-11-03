use eframe::egui;
use eframe::egui::{Context, FontId, RichText};
use eframe::Frame;

use crate::errors::errors::Error;
use crate::utils::{icon_data, label};

pub struct ErrorReportWindow {
    error: Error,
}

impl ErrorReportWindow {
    pub fn new(error: Error) -> Self {
        Self {
            error,
        }
    }

    pub fn run_window(error: Error) -> eframe::Result<()> {
        eframe::run_native(
            "Chip 8 Emulator - Error Manager",
            ErrorReportWindow::options(),
            Box::new(|_cc| {
                Ok(Box::<ErrorReportWindow>::new(
                    ErrorReportWindow::new(error)
                ))
            }),
        )
    }

    fn options() -> eframe::NativeOptions {
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([840f32, 530f32])
                .with_icon(icon_data()),
            ..Default::default()
        }
    }
}

impl eframe::App for ErrorReportWindow {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20f32);

                ui.heading(RichText::new("Error executing Chip 8 Emulator")
                    .font(FontId::proportional(40f32)));

                ui.separator();
                ui.add_space(60f32)
            });

            ui.vertical_centered(|ui| {
                ui.label(self.error.error_message.clone());

                label(format!("Error Code: {}", self.error.error_code).as_str(), ui);
            });
        });
    }
}