// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod emulator;
pub mod errors;
pub mod startup;
pub mod utils;

use std::process::exit;
use crate::emulator::window::EmulatorWindow;
use crate::errors::error_report_window::*;
use crate::startup::get_filepath;

fn main() {
    run_emulator(get_filepath());
}

fn run_emulator(filepath: String) {
    let exit_information = EmulatorWindow::run_window(filepath);
    if let Some(error) = exit_information.error {
        ErrorReportWindow::run_window(error).unwrap();
    }
}
