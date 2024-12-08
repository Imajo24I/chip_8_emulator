use std::path::PathBuf;
use eframe::egui::{Context, ViewportCommand};
use crate::emulator_app::{AppState, EmulatorApp};
use anyhow::Error;
use crate::chip_8::config::Config;
use crate::chip_8::emulator_window::EmulatorWindow;
use crate::error_report_window::ErrorReportWindow;

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
                        emulator.state = AppState::Emulating(emulator_window);
                    }

                    Err(error) => {
                        emulator.state = AppState::ErrorReporting(ErrorReportWindow::new(error));
                    }
                }
            }

            Self::ReportError(error) => {
                emulator.state = AppState::ErrorReporting(ErrorReportWindow::new(error))
            }

            Self::Exit => {
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }
        }
    }
}
