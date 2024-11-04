// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod emulator;
pub mod errors;
pub mod startup;
pub mod utils;

use std::path::PathBuf;
use crate::emulator::window::EmulatorWindow;
use crate::errors::error_report_window::*;
use crate::errors::errors::Errors;

fn main() {
    run_emulator(startup::get_filepath());
}

fn run_emulator(filepath: Option<PathBuf>) {
    if let Some(filepath) = filepath {
        let exit_information = EmulatorWindow::run_window(filepath);
        if let Some(error) = exit_information.error {
            ErrorReportWindow::run_window(error).unwrap();
        }
    } else {
        ErrorReportWindow::run_window(Errors::MissingFilePathArg.get_error()).unwrap();
    }
}
