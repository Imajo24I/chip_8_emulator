use crate::chip_8::emulator_window::EmulatorWindow;
use crate::error_report_window::ErrorReportWindow;
use crate::events::Event;
use crate::startup::StartupWindow;
use crate::utils;
use eframe::egui::{Context, FontId};
use eframe::{egui, Frame};

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
            .default_size([840f32, 640f32])
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

    fn create_emulator_window(&mut self) -> Option<Event> {
        let path = self.startup_window.startup_info.filepath.as_ref().unwrap();
        let config = self.startup_window.startup_info.config;

        let emulator_window = EmulatorWindow::new(path, config);

        match emulator_window {
            Ok(emulator_window) => {
                self.emulator_window = Some(emulator_window);
                None
            }
            Err(error) => Some(Event::ReportError(error)),
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
        } else if self.startup_window.start_emulation {
            if self.startup_window.startup_info.filepath.is_some() {
                if let Some(event) = self.create_emulator_window() {
                    event.execute(ctx, self);
                }
            } else {
                self.startup_window.start_emulation = false;
                self.startup_window(ctx);
            }
        } else {
            self.startup_window(ctx);
        }
    }
}
