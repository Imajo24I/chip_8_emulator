use eframe::egui::{Context, ViewportCommand};
use crate::emulator_app::EmulatorApp;
use crate::errors::error::Error;
use crate::errors::error_report_window::ErrorReportWindow;

pub enum Event {
    Exit,
    ReportErrorAndExit(Error)
}

impl Event {
    pub fn execute(self, ctx: &Context, chip_8_emulator: &mut EmulatorApp) {
        match self {
            Self::Exit => {
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }
            Self::ReportErrorAndExit(error) => {
                chip_8_emulator.error_report_window = Some(ErrorReportWindow::new(error))
            }
        }
    }
}