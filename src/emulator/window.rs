use eframe::{egui, Frame};
use eframe::egui::{Context, ViewportCommand};

use crate::emulator::emulator::Chip8Emulator;
use crate::errors::error_code::{Error, Errors};
use crate::errors::error_manager;
use crate::utils::icon_data;

pub struct EmulatorWindow {
    emulator: Chip8Emulator,
    filepath: String
}

impl EmulatorWindow {
    pub fn new(filepath: String) -> Self {
        Self {
            emulator: Chip8Emulator::new(filepath.clone()),
            filepath,
        }
    }

    pub fn run_window(filepath: String) -> eframe::Result<()> {
        eframe::run_native(
            "Chip 8 Emulator",
            EmulatorWindow::options(),
            Box::new(|_cc| {
                Ok(Box::<EmulatorWindow>::new(
                    EmulatorWindow::new(filepath)
                ))
            }),
        )
    }

    fn options() -> eframe::NativeOptions {
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([1280f32, 920f32])
                .with_icon(icon_data()),
            ..Default::default()
        }
    }

    fn close_with_error_report(&self, ctx: &Context, error: Error) {
        error_manager::set_error(error);
        ctx.send_viewport_cmd(ViewportCommand::Close);
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
            ui.label(format!("Given Path: {}", self.filepath));
        });
    }
}