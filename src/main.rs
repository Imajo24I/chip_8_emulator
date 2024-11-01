// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod errors;
pub mod emulator;

use std::env;
use crate::emulator::window::EmulatorWindow;
use crate::errors::error_manager::ErrorManagerWindow;
use crate::errors::error_code::{Errors::MissingFilePathArg, Error};

fn main() {
    run_window(get_filepath()).unwrap();
}

fn run_window(filepath_result: Result<String, Error>) -> eframe::Result<> {
    if let Ok(filepath) = filepath_result {
        // Run normal Chip 8 Emulator
        eframe::run_native(
            "Chip 8 Emulator",
            EmulatorWindow::options(),
            Box::new(|_cc| {
                Ok(Box::<EmulatorWindow>::new(
                    EmulatorWindow::new(filepath)
                ))
            }),
        )
    } else {
        // Run Error Manager due to missing Filepath
        let error = filepath_result.unwrap_err();

        eframe::run_native(
            "Chip 8 Emulator - Error Manager",
            ErrorManagerWindow::options(),
            Box::new(|_cc| {
                Ok(Box::<ErrorManagerWindow>::new(
                    ErrorManagerWindow::new(error)
                ))
            }),
        )
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
