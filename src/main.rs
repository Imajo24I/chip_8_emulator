// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod emulator;
pub mod errors;
pub mod startup;
pub mod utils;
pub mod chip_8_emulator;
pub mod events;

use crate::chip_8_emulator::Chip8Emulator;

fn main() {
    Chip8Emulator::run();
}
