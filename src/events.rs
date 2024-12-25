use crate::chip_8::emulator::Emulator;
use crate::emulator_app::{AppState, EmulatorApp, FONT_SIZE};
use crate::screens::emulator_screen::EmulatorScreen;
use crate::screens::error_report_screen::ErrorReportScreen;
use anyhow::Error;
use eframe::egui::{Context, FontId, ViewportCommand};

pub enum Event {
    StartEmulation(Emulator),
    ReportError(Error),
    Exit,
}

impl Event {
    pub fn execute(self, ctx: &Context, emulator_app: &mut EmulatorApp) {
        match self {
            Self::StartEmulation(emulator) => {
                ctx.style_mut(|style| {
                    style.override_font_id = None;
                });

                let emulator_screen = EmulatorScreen::new(emulator);
                emulator_app.state = AppState::Emulating(emulator_screen);
            }

            Self::ReportError(error) => {
                ctx.style_mut(|style| {
                    style.override_font_id = Some(FontId::proportional(FONT_SIZE));
                });
                emulator_app.state = AppState::ErrorReporting(ErrorReportScreen::new(error))
            }

            Self::Exit => {
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }
        }
    }
}
