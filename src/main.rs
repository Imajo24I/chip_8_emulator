// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod errors;
pub mod emulator;
pub mod utils;

use std::env;

use crate::emulator::window::EmulatorWindow;
use crate::errors::error_manager::ErrorManagerWindow;
use crate::errors::error_code::{Errors::MissingFilePathArg, Error};

fn main() {
    run_window(get_filepath()).unwrap();
}

fn run_window(filepath_result: Result<String, Error>) -> eframe::Result<> {
    if let Ok(filepath) = filepath_result {
        EmulatorWindow::run_window(filepath)
    } else {
        ErrorManagerWindow::run_window(filepath_result.unwrap_err())
    }
}

fn get_filepath() -> Result<String, Error> {
    let filepath;
    let args = env::args();

    if args.len() > 1 {
        let args: Vec<String> = args.collect();
        filepath = args[1].to_owned();
    } else {
        return Err(
            MissingFilePathArg.get_error()
        );
    }

    Ok(filepath)
}
