use crate::chip_8::config::Config;
use crate::screens::emulator_screen::EmulatorScreen;
use crate::emulator_app::{AppState, EmulatorApp, FONT_SIZE};
use crate::screens::error_report_screen::ErrorReportScreen;
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
                let emulator_screen = EmulatorScreen::new(&filepath, config);

                match emulator_screen {
                    Ok(emulator_screen) => {
                        set_default_font(ctx);
                        emulator.state = AppState::Emulating(emulator_screen);
                    }

                    Err(error) => {
                        set_custom_font(ctx);
                        emulator.state = AppState::ErrorReporting(ErrorReportScreen::new(error));
                    }
                }
            }

            Self::ReportError(error) => {
                set_custom_font(ctx);
                emulator.state = AppState::ErrorReporting(ErrorReportScreen::new(error))
            }

            Self::Exit => {
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }
        }
    }
}

fn set_default_font(ctx: &Context) {
    ctx.style_mut(|style| {
        style.override_font_id = None;
    });
}

fn set_custom_font(ctx: &Context) {
    ctx.style_mut(|style| {
        style.override_font_id = Some(FontId::proportional(FONT_SIZE));
    });
}
