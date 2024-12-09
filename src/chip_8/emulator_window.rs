use crate::chip_8::config::Config;
use crate::chip_8::emulator::Emulator;
use crate::events::Event;
use anyhow::Error;
use eframe::egui;
use eframe::egui::{Pos2, Ui, Vec2};
use std::path::Path;
use std::time::{Duration, Instant};

pub struct EmulatorWindow {
    emulator: Emulator,
    config: Config,
    next_frame: Instant,
}

impl EmulatorWindow {
    pub fn new(filepath: &Path, config: Config) -> Result<Self, Error> {
        Ok(Self {
            emulator: Emulator::new(filepath, config)?,
            config,
            next_frame: Instant::now(),
        })
    }

    pub fn update(&mut self, ui: &mut Ui) -> Option<Event> {
        ui.ctx().request_repaint();

        self.wait_for_next_frame();

        ui.input(|input_state| {
            self.emulator.keypad.update_keys(input_state);
        });

        self.emulator.tick_timers();

        for _ in 0..self.config.cycles_per_frame {
            if let Err(event) = self.emulator.run_cycle() {
                return Some(event);
            }
        }

        let inner_rect = ui.ctx().input(|i| i.viewport().inner_rect);
        if let Some(inner_rect) = inner_rect {
            self.draw_display(inner_rect.size(), ui);
        }

        None
    }

    fn draw_display(&mut self, window_size: Vec2, ui: &mut Ui) {
        let pixel_width = window_size.x / 64f32;
        let pixel_height = window_size.y / 32f32;

        for (row_index, row) in self.emulator.display.iter().enumerate() {
            let pixel_y = row_index as f32 * pixel_height;

            for (pixel_index, pixel) in row.iter().enumerate() {
                let pixel_pos = Pos2::new(pixel_index as f32 * pixel_width, pixel_y);

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
    }

    fn wait_for_next_frame(&mut self) {
        let now = Instant::now();
        self.next_frame += Duration::from_secs_f32(1f32 / 60f32);
        std::thread::sleep(self.next_frame - now);
    }
}
