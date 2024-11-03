use eframe::egui::{Context, ViewportCommand};
use eframe::{egui, Frame};

use crate::emulator::emulator::Chip8Emulator;
use crate::errors::error_code::{Error, Errors};
use crate::utils::icon_data;

pub struct EmulatorWindow<'a> {
    emulator: Chip8Emulator,
    filepath: String,
    exit_information: &'a mut ExitInformation,
}

impl<'a> EmulatorWindow<'a> {
    pub fn new(filepath: String, exit_information: &'a mut ExitInformation) -> Self {
        Self {
            emulator: Chip8Emulator::new(filepath.clone()),
            filepath,
            exit_information,
        }
    }

    pub fn run_window(filepath: String) -> ExitInformation {
        let mut exit_information = ExitInformation::default();

        eframe::run_native(
            "Chip 8 Emulator",
            EmulatorWindow::options(),
            Box::new(|_cc| {
                Ok(Box::<EmulatorWindow>::new(
                    EmulatorWindow::new(filepath, &mut exit_information)
                ))
            }),
        ).unwrap();

        exit_information
    }

    fn options() -> eframe::NativeOptions {
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([1280f32, 920f32])
                .with_icon(icon_data()),
            ..Default::default()
        }
    }

    fn exit_with_error(&mut self, ctx: &Context, error: Error) {
        self.exit_information.error = Some(error);
        ctx.send_viewport_cmd(ViewportCommand::Close);
    }
}

impl eframe::App for EmulatorWindow<'_> {
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

pub struct ExitInformation {
    pub error: Option<Error>,
}

impl Default for ExitInformation {
    fn default() -> Self {
        Self {
            error: None
        }
    }
}