use crate::emulator::emulator::Emulator;
use crate::events::Event;
use eframe::egui;
use eframe::egui::{Pos2, Ui};
use crate::errors::error::Error;
use std::path::Path;

const PIXEL_WIDTH: f32 = 20f32;

pub struct EmulatorWindow {
    emulator: Emulator,
}

impl EmulatorWindow {
    pub fn new(filepath: &Path) -> Result<Self, Error> {
        Ok(Self {
            emulator: Emulator::new(filepath)?,
        })
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
