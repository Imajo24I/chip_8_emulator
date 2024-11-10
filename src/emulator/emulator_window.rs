use eframe::egui;
use eframe::egui::{Pos2, Ui};
use std::path::Path;

use crate::emulator::emulator::Chip8Emulator;
use crate::events::Event;

const PIXEL_WIDTH: f32 = 20f32;

pub struct EmulatorWindow {
    emulator: Chip8Emulator,
}

impl EmulatorWindow {
    pub fn new(filepath: &Path) -> Self {
        Self {
            emulator: Chip8Emulator::new(filepath),
        }
    }

    pub fn update(&mut self, ui: &mut Ui) -> Option<Event> {
        // Draw Display
        for (row_index, row) in self.emulator.display.iter().enumerate() {
            for (pixel_index, pixel) in row.iter().enumerate() {
                let pixel_pos = Pos2::new(pixel_index as f32 * PIXEL_WIDTH, row_index as f32 * PIXEL_WIDTH);

                let rect = egui::Rect::from_two_pos(
                    pixel_pos,
                    pixel_pos + egui::vec2(PIXEL_WIDTH, PIXEL_WIDTH),
                );

                let rect_color = if *pixel {
                    egui::Color32::WHITE
                } else {
                    egui::Color32::BLACK
                };

                ui.painter().rect_filled(rect, 0f32, rect_color);
            }
        }

        None
    }
}
