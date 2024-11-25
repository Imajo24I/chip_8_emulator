// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod chip_8;
pub mod errors;
pub mod startup;
pub mod utils;
pub mod emulator_app;
pub mod events;

use crate::emulator_app::EmulatorApp;

// TODO: Make this configurable
const USE_GERMAN_KEYBOARD_LAYOUT: bool = true;

fn main() {
    EmulatorApp::run();
}
