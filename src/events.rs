use crate::chip_8::config::Config;
use crate::chip_8::emulator_window::EmulatorWindow;
use crate::emulator_app::{AppState, EmulatorApp, FONT_SIZE};
use crate::error_report_window::ErrorReportWindow;
use anyhow::Error;
use eframe::egui::{Context, FontId, ViewportCommand};
use std::path::PathBuf;

pub enum Event {
    StartEmulation(PathBuf, Config),
    ReportError(Error),
    Exit,
}

impl Event {
    pub fn execute(self, ctx: &Context, emulator: &mut EmulatorApp) {
        match self {
            Self::StartEmulation(filepath, config) => {
                let emulator_window = EmulatorWindow::new(&filepath, config);

                match emulator_window {
                    Ok(emulator_window) => {
                        ctx.style_mut(|style| {
                            style.override_font_id = None;
                        });
                        emulator.state = AppState::Emulating(emulator_window);
                    }

                    Err(error) => {
                        ctx.style_mut(|style| {
                            style.override_font_id = Some(FontId::proportional(FONT_SIZE));
                        });
                        emulator.state = AppState::ErrorReporting(ErrorReportWindow::new(error));
                    }
                }
            }

            Self::ReportError(error) => {
                ctx.style_mut(|style| {
                    style.override_font_id = Some(FontId::proportional(FONT_SIZE));
                });
                emulator.state = AppState::ErrorReporting(ErrorReportWindow::new(error))
            }

            Self::Exit => {
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }
        }
    }
}
