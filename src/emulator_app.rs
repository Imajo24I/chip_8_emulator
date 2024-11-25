use eframe::egui::{Context, FontId};
use eframe::{egui, Frame};
use std::path::Path;

use crate::emulator::emulator_window::EmulatorWindow;
use crate::errors::error_report_window::ErrorReportWindow;
use crate::events::Event;
use crate::startup::StartupWindow;
use crate::utils;

pub const FONT_SIZE: f32 = 20f32;

#[derive(Default)]
pub struct EmulatorApp {
    pub startup_window: StartupWindow,
    pub emulator_window: Option<EmulatorWindow>,
    pub error_report_window: Option<ErrorReportWindow>,
}

impl EmulatorApp {
    pub fn run() {
        eframe::run_native(
            "Chip 8 Emulator",
            Self::options(),
            Box::new(|cc| {
                cc.egui_ctx.style_mut(|style| {
                    style.override_font_id = Some(FontId::proportional(FONT_SIZE));
                });

                Ok(Box::new(Self::default()))
            }),
        )
        .unwrap();
    }

    fn options() -> eframe::NativeOptions {
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([1280f32, 640f32])
                .with_icon(utils::icon_data()),
            ..Default::default()
        }
    }

    fn startup_window(&mut self, ctx: &Context) {
        egui::Window::new("Startup")
            .collapsible(false)
            .default_size([840f32, 6400f32])
            .show(ctx, |ui| {
                self.startup_window.update(ui);
            });
    }

    fn emulator_window(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let event = self.emulator_window.as_mut().unwrap().update(ui);

            if let Some(event) = event {
                event.execute(ctx, self);
            }
        });
    }

    fn create_emulator_window(&mut self, path: &Path) -> Option<Event> {
        let emulator_window = EmulatorWindow::new(path);

        match emulator_window {
            Ok(emulator_window) => {
                self.emulator_window = Some(emulator_window);
                None
            }
            Err(error) => Some(Event::ReportErrorAndExit(error)),
        }
    }

    fn error_report_window(&mut self, ctx: &Context) {
        egui::Window::new("Error executing Chip 8 Emulator")
            .default_size([830f32, 830f32])
            .collapsible(false)
            .show(ctx, |ui| {
                let event = self.error_report_window.as_mut().unwrap().update(ui);

                if let Some(event) = event {
                    event.execute(ctx, self);
                }
            });
    }
}

impl eframe::App for EmulatorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if self.error_report_window.is_some() {
            self.error_report_window(ctx);
        } else if self.emulator_window.is_some() {
            self.emulator_window(ctx);
        } else if self.startup_window.use_selected_filepath {
            if let Some(filepath) = &self.startup_window.startup_info.filepath {
                if let Some(event) = self.create_emulator_window(&filepath.clone()) {
                    event.execute(ctx, self);
                }
            } else {
                self.startup_window.use_selected_filepath = false;
                self.startup_window(ctx);
            }
        } else {
            self.startup_window(ctx);
        }
    }
}
