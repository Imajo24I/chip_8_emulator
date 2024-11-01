use eframe::egui;
use eframe::egui::{Context, FontId, RichText};
use eframe::Frame;

use crate::errors::error_code::Error;
use crate::utils::icon_data;

pub struct ErrorManagerWindow {
    error: Error,
}

impl ErrorManagerWindow {
    pub fn new(error: Error) -> Self {
        Self {
            error,
        }
    }

    pub fn run_window(error: Error) -> eframe::Result<()> {
        eframe::run_native(
            "Chip 8 Emulator - Error Manager",
            ErrorManagerWindow::options(),
            Box::new(|_cc| {
                Ok(Box::<ErrorManagerWindow>::new(
                    ErrorManagerWindow::new(error)
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

impl eframe::App for ErrorManagerWindow {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20f32);

                ui.heading(RichText::new("Error executing Chip 8 Emulator").font(FontId::proportional(40f32)));

                ui.separator();
                ui.add_space(60f32)
            });

            ui.vertical_centered(|ui| {
                ui.label(self.error.error_message.clone());

                ui.label(RichText::new(
                    format!("Error Code: {}", self.error.error_code))
                    .font(FontId::proportional(20f32)
                    ));
            });
        });
    }
}