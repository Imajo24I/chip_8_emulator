use std::path::PathBuf;
use eframe::egui::{Context, Pos2, Ui, ViewportCommand};
use eframe::{egui, Frame};

use crate::emulator::emulator::Chip8Emulator;
use crate::errors::error::Error;
use crate::utils;

const PIXEL_WIDTH: f32 = 20f32;

pub struct EmulatorWindow<'a> {
    emulator: Chip8Emulator,
    exit_information: &'a mut ExitInformation,
}

impl<'a> EmulatorWindow<'a> {
    pub fn new(filepath: PathBuf, exit_information: &'a mut ExitInformation) -> Self {
        Self {
            emulator: Chip8Emulator::new(filepath),
            exit_information,
        }
    }

    pub fn run_window(filepath: PathBuf) -> ExitInformation {
        let mut exit_information = ExitInformation::default();

        eframe::run_native(
            "Chip 8 Emulator",
            EmulatorWindow::options(),
            Box::new(|cc| {
                utils::set_default_style(cc);

                Ok(Box::<EmulatorWindow>::new(
                    EmulatorWindow::new(filepath, &mut exit_information)
                ))
            }),
        ).unwrap();

        exit_information
    }

    fn options() -> eframe::NativeOptions {
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([1280f32, 640f32])
                .with_icon(utils::icon_data()),
            ..Default::default()
        }
    }

    fn exit_with_error(&mut self, error: Error, ctx: &Context) {
        self.exit_information.error = Some(error);
        ctx.send_viewport_cmd(ViewportCommand::Close);
    }

    fn draw_display(&self, ui: &mut Ui) {
        for (row_index, row) in self.emulator.display.iter().enumerate() {
            for (pixel_index, pixel) in row.iter().enumerate() {
                let pixel_pos = Pos2::new(pixel_index as f32 * PIXEL_WIDTH, row_index as f32 * PIXEL_WIDTH);

                let rect = egui::Rect::from_two_pos(
                    pixel_pos,
                    pixel_pos + egui::vec2(PIXEL_WIDTH, PIXEL_WIDTH)
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
}

impl eframe::App for EmulatorWindow<'_> {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // Get delta time from input state
        let mut dt = 0f32;
        ctx.input(|input_state| {
            dt = input_state.stable_dt;
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_display(ui);
        });
    }
}

#[derive(Default)]
pub struct ExitInformation {
    pub error: Option<Error>,
}
