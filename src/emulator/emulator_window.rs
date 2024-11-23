use crate::emulator::emulator::Emulator;
use crate::errors::error::Error;
use crate::events::Event;
use eframe::egui;
use eframe::egui::{Pos2, Ui};
use std::path::Path;
use std::time::{Duration, Instant};

const DURATION_PER_CYCLE: Duration = Duration::from_millis(1000 / 60);

pub struct EmulatorWindow {
    emulator: Emulator,
    last_cycle: Instant,
}

impl EmulatorWindow {
    pub fn new(filepath: &Path) -> Result<Self, Error> {
        Ok(Self {
            emulator: Emulator::new(filepath)?,
            last_cycle: Instant::now(),
        })
    }

    pub fn update(&mut self, ui: &mut Ui) -> Option<Event> {
        ui.ctx().request_repaint();

        if self.last_cycle.elapsed() >= DURATION_PER_CYCLE {
            let mut event = None;

            ui.input(|input_state| {
                if let Err(returned_event) = self.emulator.run_cycle(input_state) {
                    event = Some(returned_event);
                }
            });

            self.last_cycle = Instant::now();
        }

        let window_size = ui.ctx().input(|i| i.viewport().inner_rect.unwrap().size());
        let pixel_width = window_size.x / 64f32;
        let pixel_height = window_size.y / 32f32;

        // Draw Display
        for (row_index, row) in self.emulator.display.iter().enumerate() {
            for (pixel_index, pixel) in row.iter().enumerate() {
                let pixel_pos = Pos2::new(
                    pixel_index as f32 * pixel_width,
                    row_index as f32 * pixel_height,
                );

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
