use crate::chip_8::emulator::Emulator;
use crate::emulator_app::Event;
use crate::screens::emulator_settings::draw_settings;
use eframe::egui::{Context, Ui};
use std::path::PathBuf;

pub struct StartupScreen {
    pub filepath: Option<PathBuf>,
    pub emulator: Emulator,
}

impl StartupScreen {
    fn collect_dropped_files(&mut self, ctx: &Context) {
        ctx.input(|input| {
            if !input.raw.dropped_files.is_empty() {
                self.filepath = Some(input.raw.dropped_files[0].path.clone().unwrap());
            }
        });
    }

    fn file_dialog(&mut self, ui: &mut Ui) {
        if ui.button("Select File...").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                self.filepath = Some(path);
            }
        }
    }

    pub fn update(&mut self, ui: &mut Ui) -> Option<Event> {
        ui.vertical_centered(|ui| {
            self.update_filepath(ui);

            draw_settings(ui, &mut self.emulator);

            ui.end_row();
            ui.add_space(20f32);

            if ui.button("Start Emulation").clicked() && self.filepath.is_some() {
                let result = self.emulator.load_rom(self.filepath.as_ref().unwrap());

                return if let Err(error) = result {
                    Some(Event::ReportError(error))
                } else {
                    Some(Event::StartEmulation(self.emulator.clone()))
                };
            }

            ui.end_row();
            ui.add_space(20f32);

            None
        })
        .inner
    }

    fn update_filepath(&mut self, ui: &mut Ui) {
        ui.add_space(10f32);

        ui.label("Selected File:").on_hover_text(
            "Select a file by dragging and dropping it here or by using the file dialog below.",
        );

        if let Some(filepath) = &self.filepath {
            ui.label(filepath.file_name().unwrap().to_string_lossy());
        } else {
            ui.label("No file selected").on_hover_text(
                "Select a file by dragging and dropping it here or by using the file dialog below.",
            );
        }

        ui.end_row();
        ui.add_space(10f32);

        self.file_dialog(ui);

        ui.end_row();
        ui.add_space(20f32);

        self.collect_dropped_files(ui.ctx());
    }
}

impl Default for StartupScreen {
    fn default() -> Self {
        let mut filepath = None;

        let mut args = std::env::args();
        if args.len() > 1 {
            filepath = Some(PathBuf::from(args.nth(1).unwrap()));
        }

        Self {
            filepath,
            emulator: Emulator::default(),
        }
    }
}
