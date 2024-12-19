use crate::chip_8::config::Config;
use crate::chip_8::emulator::Emulator;
use crate::events::Event;
use anyhow::Error;
use eframe::egui;
use eframe::egui::{Button, Label, Pos2, Rect, RichText, Ui, Vec2};
use std::path::Path;
use std::time::{Duration, Instant};

pub const MENU_BAR_OFFSET: f32 = 30f32;

pub struct EmulatorWindow {
    emulator: Emulator,
    config: Config,
    next_frame: Instant,
    sleep_time: Duration
}

impl EmulatorWindow {
    pub fn new(filepath: &Path, config: Config) -> Result<Self, Error> {
        Ok(Self {
            emulator: Emulator::new(filepath, config)?,
            config,
            next_frame: Instant::now(),
            sleep_time: Duration::from_secs(0)
        })
    }

    pub fn update(&mut self, ui: &mut Ui) -> Option<Event> {
        ui.ctx().request_repaint();

        self.wait_for_next_frame();

        if !self.config.emulation_paused {
            ui.input(|input_state| {
                self.emulator.keypad.update_keys(input_state);
            });

            self.emulator.tick_timers();

            for _ in 0..self.config.cycles_per_frame {
                if let Err(event) = self.emulator.run_cycle() {
                    return Some(event);
                }
            }
        }

        let inner_rect = ui.ctx().input(|i| i.viewport().inner_rect);
        if let Some(inner_rect) = inner_rect {
            let window_size = inner_rect.size();

            self.draw_display(window_size, ui);
            return self.draw_menu_bar(window_size, ui);
        }

        None
    }

    fn draw_display(&mut self, window_size: Vec2, ui: &mut Ui) {
        let pixel_width = window_size.x / 64f32;
        let pixel_height = (window_size.y - MENU_BAR_OFFSET) / 32f32;

        for (row_index, row) in self.emulator.display.iter().enumerate() {
            let pixel_y = row_index as f32 * pixel_height;

            for (pixel_index, pixel) in row.iter().enumerate() {
                let pixel_pos = Pos2::new(pixel_index as f32 * pixel_width, pixel_y);

                let rect = Rect::from_two_pos(
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

    fn draw_menu_bar(&mut self, window_size: Vec2, ui: &mut Ui) -> Option<Event> {
        let bar_height = window_size.y - MENU_BAR_OFFSET + 5f32;
        let bar_top_height = window_size.y - 5f32;
        let window_center = window_size.x / 2f32;

        if ui
            .put(
                Rect::from_two_pos(
                    Pos2::new(window_center - 50f32, bar_height),
                    Pos2::new(window_center + 50f32, bar_top_height),
                ),
                Button::new(RichText::new("Open Menu")),
            )
            .clicked()
        {}

        if ui
            .put(
                Rect::from_two_pos(
                    Pos2::new(window_center - 200f32, bar_height),
                    Pos2::new(window_center - 100f32, bar_top_height),
                ),
                Button::new("Resume/Pause"),
            )
            .clicked()
        {
            self.config.emulation_paused = !self.config.emulation_paused;
        }

        if ui
            .put(
                Rect::from_two_pos(
                    Pos2::new(window_center + 100f32, bar_height),
                    Pos2::new(window_center + 200f32, bar_top_height),
                ),
                Button::new("Exit Emulation"),
            )
            .clicked()
        {
            return Some(Event::Exit);
        }

        ui.put(
            Rect::from_two_pos(
                Pos2::new(100f32, bar_height - 2f32),
                Pos2::new(300f32, bar_top_height),
            ),
            Label::new(if self.config.emulation_paused {
                "Emulation Paused"
            } else {
                "Emulation Running"
            }),
        );

        let frame_time = ui.ctx().input(|input| input.stable_dt * 1000f32)
            - self.sleep_time.as_millis() as f32;

        ui.put(
            Rect::from_two_pos(
                Pos2::new(window_size.x - 400f32, bar_height - 2f32),
                Pos2::new(window_size.x - 100f32, bar_top_height),
            ),
            Label::new(format!("Frame Time: {:.2}ms", frame_time)),
        );

        None
    }

    fn wait_for_next_frame(&mut self) {
        self.next_frame += Duration::from_secs_f32(1f32 / 60f32);

        let sleep_time = self.next_frame - Instant::now();
        self.sleep_time = sleep_time;
        std::thread::sleep(sleep_time);
    }
}
