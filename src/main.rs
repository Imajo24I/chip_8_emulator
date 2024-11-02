// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod emulator;
pub mod errors;
pub mod startup;
pub mod utils;

use crate::emulator::window::EmulatorWindow;
use crate::errors::error_manager::ErrorManagerWindow;
use crate::startup::get_filepath;

fn main() {
    run_emulator(get_filepath()).unwrap();
}

fn run_emulator(filepath: String) -> eframe::Result<> {
    EmulatorWindow::run_window(filepath)?;
    ErrorManagerWindow::run_if_error()
}
