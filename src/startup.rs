use crate::chip_8::config::Config;
use crate::emulator_app::FONT_SIZE;
use crate::utils;
use eframe::egui::{Align, Context, RichText, TextEdit, Ui};
use std::env;
use std::path::PathBuf;

#[derive(Default)]
pub struct StartupWindow {
    pub startup_info: StartUpInfo,
    pub start_emulation: bool,
    looked_in_args: bool,
}

impl StartupWindow {
    fn collect_dropped_files(&mut self, ctx: &Context) {
        ctx.input(|input| {
            if !input.raw.dropped_files.is_empty() {
                self.startup_info.filepath = Some(input.raw.dropped_files[0].path.clone().unwrap());
            }
        });
    }

    fn file_dialog(&mut self, ui: &mut Ui) {
        if utils::button("Select File...", ui).clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                self.startup_info.filepath = Some(path);
            }
        }
    }

    pub fn update(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            self.update_filepath(ui);

            ui.end_row();
            ui.add_space(20f32);

            self.update_emulator_config(ui);

            ui.end_row();
            ui.add_space(20f32);

            if utils::button("Start Emulation", ui).clicked() {
                self.start_emulation = true;
            }

            ui.end_row();
            ui.add_space(20f32);
        });
    }

    fn update_filepath(&mut self, ui: &mut Ui) {
        if !self.looked_in_args {
            self.looked_in_args = true;
            if let Some(filepath) = Self::filepath_from_args() {
                self.startup_info.filepath = Some(filepath);
                self.start_emulation = true;
            }
        }

        ui.add_space(10f32);

        ui.label("Selected File:");

        ui.end_row();

        if let Some(filepath) = &self.startup_info.filepath {
            ui.label(filepath.to_str().unwrap());
        } else {
            ui.label("No file selected");
        }

        ui.end_row();
        ui.add_space(10f32);

        self.file_dialog(ui);

        ui.end_row();
        ui.add_space(20f32);

        self.collect_dropped_files(ui.ctx());
    }

    fn filepath_from_args() -> Option<PathBuf> {
        let args = env::args();

        if args.len() > 1 {
            let args: Vec<String> = args.collect();
            Some(PathBuf::from(args[1].to_owned()))
        } else {
            None
        }
    }

    pub fn update_emulator_config(&mut self, ui: &mut Ui) {
        ui.checkbox(
            &mut self.startup_info.config.use_german_keyboard_layout,
            RichText::new("Use german keyboard layout").size(FONT_SIZE),
        );

        ui.end_row();
        ui.add_space(20f32);

        ui.label("Cycles per Frame:");
        ui.add_space(5f32);

        let cycles_per_frame = &mut self.startup_info.config.cycles_per_frame;
        let mut text = cycles_per_frame.to_string();

        if ui
            .add_sized(
                [100f32, 20f32],
                TextEdit::singleline(&mut text).horizontal_align(Align::Center),
            )
            .changed()
        {
            let mut value = text.parse::<u32>().unwrap_or(*cycles_per_frame);

            if text.is_empty() {
                value = 0;
            }

            *cycles_per_frame = if value > 9999 {
                *cycles_per_frame
            } else {
                value
            };
        }
    }
}

#[derive(Default)]
pub struct StartUpInfo {
    pub filepath: Option<PathBuf>,
    pub config: Config,
}
