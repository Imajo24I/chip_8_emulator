use eframe::egui::{Context, ViewportCommand};
use crate::emulator_app::EmulatorApp;
use anyhow::Error;
use crate::error_report_window::ErrorReportWindow;

pub enum Event {
    Exit,
    ReportError(Error)
}

impl Event {
    pub fn execute(self, ctx: &Context, chip_8_emulator: &mut EmulatorApp) {
        match self {
            Self::Exit => {
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }
            Self::ReportError(error) => {
                chip_8_emulator.error_report_window = Some(ErrorReportWindow::new(error))
            }
        }
    }
}