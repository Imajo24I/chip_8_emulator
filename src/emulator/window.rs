use eframe::{egui, Frame};
use eframe::egui::Context;

use crate::emulator::emulator::Chip8Emulator;

pub struct EmulatorWindow {
    emulator: Chip8Emulator,
}

impl EmulatorWindow {
    pub fn new(filepath: String) -> Self {
        Self {
            emulator: Chip8Emulator::new(filepath),
        }
    }

    pub fn options() -> eframe::NativeOptions {
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([1280f32, 920f32]),
            ..Default::default()
        }
    }
}

impl eframe::App for EmulatorWindow {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // Get delta time from input state
        let mut dt = 0f32;
        ctx.input(|input_state| {
            dt = input_state.stable_dt;
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut 5, 0..=10).text("Some Value"))
        });
    }
}