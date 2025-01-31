use crate::chip_8::emulator::Emulator;
use crate::emulator_app::FONT_SIZE;
use crate::events::Event;
use crate::screens::emulator_settings::draw_settings;
use eframe::egui;
use eframe::egui::{
    Button, Color32, FontId, Image, Label, Pos2, Rect, RichText, TextureOptions, Ui, Vec2, Window,
};
use std::time::{Duration, Instant};

pub const MENU_BAR_OFFSET: f32 = 30f32;
pub const TEXTURE_OPTIONS: TextureOptions = TextureOptions {
    magnification: egui::TextureFilter::Nearest,
    minification: egui::TextureFilter::Nearest,
    wrap_mode: egui::TextureWrapMode::ClampToEdge,
    mipmap_mode: None,
};

pub struct EmulatorScreen {
    emulator: Emulator,
    next_frame: Instant,
    sleep_time: Duration,
    settings_opened: bool,
}

impl EmulatorScreen {
    pub fn new(emulator: Emulator) -> Self {
        Self {
            emulator,
            next_frame: Instant::now(),
            sleep_time: Duration::from_secs(0),
            settings_opened: false,
        }
    }

    pub fn update(&mut self, ui: &mut Ui) -> Option<Event> {
        ui.ctx().request_repaint();

        self.wait_for_next_frame();

        if !self.emulator.config.emulation_paused {
            ui.input(|input_state| {
                self.emulator.keypad.update_keys(input_state);
            });

            self.emulator.tick_timers();

            for _ in 0..self.emulator.config.instructions_per_frame {
                if let Err(event) = self.emulator.execute_instruction() {
                    return Some(event);
                }
            }
        }

        let inner_rect = ui
            .ctx()
            .input(|input_state| input_state.viewport().inner_rect);

        if let Some(inner_rect) = inner_rect {
            let window_size = inner_rect.size();

            self.draw_display(window_size, ui);
            return self.draw_menu_bar(window_size, ui);
        }

        None
    }

    /// Draw the program display
    ///
    /// Creates a `ColorImage` from the display pixels and loads it into a texture handle.
    /// Then uses the texture handle to paint an image of the display on the screen.
    fn draw_display(&mut self, window_size: Vec2, ui: &mut Ui) {
        let width = self.emulator.display.resolution.width();
        let height = self.emulator.display.resolution.height();

        let mut image_data: Vec<u8> = Vec::with_capacity(width * height * 4);

        for row in self.emulator.display.zip_planes() {
            for pixel in row {
                let color = match pixel {
                    (false, false) => Color32::BLACK,
                    (true, false) => Color32::WHITE,
                    (false, true) => Color32::LIGHT_GREEN,
                    (true, true) => Color32::DARK_GREEN,
                };

                image_data.extend_from_slice(&[color.r(), color.g(), color.b(), color.a()]);
            }
        }

        let color_image = egui::ColorImage::from_rgba_unmultiplied([width, height], &image_data);
        let texture_handle = ui
            .ctx()
            .load_texture("display_texture", color_image, TEXTURE_OPTIONS);

        Image::from(&texture_handle).paint_at(
            ui,
            Rect::from_two_pos(
                Pos2::ZERO,
                Pos2::new(window_size.x, window_size.y - MENU_BAR_OFFSET),
            ),
        );
    }

    /// Draw the bottom menu bar
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
                Button::new(RichText::new("Open Settings")),
            )
            .clicked()
        {
            self.emulator.beeper.pause();
            self.emulator.config.emulation_paused = true;
            self.settings_opened = true;
        }

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
            if self.emulator.config.emulation_paused {
                self.emulator.beeper.play();
                self.emulator.config.emulation_paused = false;
            } else {
                self.emulator.beeper.pause();
                self.emulator.config.emulation_paused = true;
            }
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
            Label::new(if self.emulator.config.emulation_paused {
                "Emulation Paused"
            } else {
                "Emulation Running"
            }),
        );

        let frame_time =
            ui.ctx().input(|input| input.stable_dt * 1000f32) - self.sleep_time.as_millis() as f32;

        ui.put(
            Rect::from_two_pos(
                Pos2::new(window_size.x - 400f32, bar_height - 2f32),
                Pos2::new(window_size.x - 100f32, bar_top_height),
            ),
            Label::new(format!("Frame Time: {:.2}ms", frame_time)),
        )
        .on_hover_text(
            "The frame time is how long a full frame takes, without considering the FPS limiting.",
        );

        if self.settings_opened {
            Window::new("Settings")
                .default_size([440f32, 320f32])
                .collapsible(false)
                .show(ui.ctx(), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.style_mut().override_font_id = Some(FontId::proportional(FONT_SIZE));

                        draw_settings(ui, &mut self.emulator);

                        ui.end_row();
                        ui.add_space(20f32);

                        if ui.button("Close Settings").clicked() {
                            self.emulator.beeper.play();
                            self.emulator.config.emulation_paused = false;
                            self.settings_opened = false;
                        }
                    });
                });
        }

        None
    }

    /// Sleep until the next frame begins, in order to limit the frame rate to 60 FPS
    fn wait_for_next_frame(&mut self) {
        self.next_frame += Duration::from_secs_f32(1f32 / 60f32);

        let sleep_time = self.next_frame - Instant::now();
        self.sleep_time = sleep_time;
        std::thread::sleep(sleep_time);
    }
}
