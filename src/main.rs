// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod chip_8;
pub mod startup;
pub mod emulator_app;
pub mod events;
pub mod error_report_window;

use crate::emulator_app::EmulatorApp;

fn main() {
    EmulatorApp::run();
}
