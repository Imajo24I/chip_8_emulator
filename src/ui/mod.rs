mod main_screen;
mod settings;

use crate::chip_8::emulator::Emulator;
use crate::emulator_app::{Event, FrameData, FONT_SIZE};
use crate::ui::main_screen::MainScreen;
use anyhow::Error;
use eframe::egui;
use eframe::egui::{Context, FontId};
use settings::Settings;
use std::cell::RefCell;
use std::rc::Rc;

pub const MENU_BAR_OFFSET: f32 = 30.0;
const TEXTURE_OPTIONS: egui::TextureOptions = egui::TextureOptions {
    magnification: egui::TextureFilter::Nearest,
    minification: egui::TextureFilter::Nearest,
    wrap_mode: egui::TextureWrapMode::ClampToEdge,
    mipmap_mode: None,
};

pub struct Screen {
    main_screen: MainScreen,
    settings: Settings,
}

impl Screen {
    pub fn new(emulator: Rc<RefCell<Emulator>>, frame_data: Rc<RefCell<FrameData>>) -> Self {
        let main_screen = MainScreen::new(emulator.clone(), frame_data);
        let settings = Settings::new(emulator);

        Self {
            main_screen,
            settings,
        }
    }

    pub fn draw_main_screen(&mut self, ctx: &Context, paused: bool) -> Option<Event> {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.style_mut().override_font_id = None;

                let inner_rect = ui.ctx().input(|input| input.viewport().inner_rect);

                let event = if let Some(inner_rect) = inner_rect {
                    let window_size = inner_rect.size();
                    self.main_screen.draw_main_screen(ui, window_size, paused)
                } else {
                    None
                };

                ui.style_mut().override_font_id = Some(FontId::proportional(FONT_SIZE));

                event
            })
            .inner
    }

    pub fn draw_settings(&mut self, ctx: &Context) -> Option<Event> {
        egui::Window::new("Settings")
            .collapsible(false)
            .default_size([440.0, 320.0])
            .default_pos(egui::Pos2::new(
                (ctx.screen_rect().size().x - 440.0) / 2.0,
                30.0,
            ))
            .show(ctx, |ui| self.settings.draw_settings(ui))
            .unwrap()
            .inner
            .unwrap()
    }

    pub fn draw_error(&mut self, ctx: &Context, error: &Error) -> Option<Event> {
        egui::Window::new("Error trying to run ROM")
            .collapsible(false)
            .default_size([830f32, 830f32])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20f32);
                    ui.label(error.to_string());

                    if let Some(source) = error.source() {
                        ui.add_space(20f32);
                        ui.label(source.to_string());
                    }

                    ui.add_space(20f32);
                    ui.separator();
                    ui.add_space(10f32);

                    if ui.button("Exit Emulator").clicked() {
                        Some(Event::Exit)
                    } else {
                        None
                    }
                })
                .inner
            })
            .unwrap()
            .inner
            .unwrap()
    }
}
