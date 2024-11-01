use eframe::egui;
use eframe::egui::{Context, FontId, IconData, ImageData, RichText, Visuals};
use eframe::Frame;

use crate::errors::error_code::Error;

pub struct ErrorManagerWindow {
    error: Error,
}

impl ErrorManagerWindow {
    pub fn new(error: Error) -> Self {
        Self {
            error,
        }
    }

    pub fn options() -> eframe::NativeOptions {
        let icon_data = eframe::icon_data::from_png_bytes(
            include_bytes!("../../assets/icon.png")
        ).expect("Failed to load icon.");

        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([840f32, 530f32])
                .with_icon(icon_data),
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