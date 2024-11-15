use eframe::egui;
use eframe::egui::{Pos2, Ui};
use std::path::Path;

use crate::emulator::emulator::Emulator;
use crate::events::Event;
use crate::errors::error::Error;

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
        let window_size = ui.ctx().input(|i| i.viewport().inner_rect.unwrap().size());
        let pixel_width = window_size.x / 64f32;
        let pixel_height = window_size.y / 32f32;

        // Draw Display
        for (row_index, row) in self.emulator.display.iter().enumerate() {
            for (pixel_index, pixel) in row.iter().enumerate() {
                let pixel_pos = Pos2::new(pixel_index as f32 * pixel_width, row_index as f32 * pixel_height);

                let rect = egui::Rect::from_two_pos(
                    pixel_pos,
                    pixel_pos + egui::vec2(pixel_width + 1f32, pixel_height + 1f32),
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
