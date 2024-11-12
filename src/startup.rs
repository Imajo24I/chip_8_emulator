use crate::utils;
use eframe::egui::{Context, Ui};
use std::env;
use std::path::PathBuf;


#[derive(Default)]
pub struct StartupWindow {
    pub startup_info: StartUpInfo,
    pub use_selected_filepath: bool,
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
        if utils::button("Open File...", ui).clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                self.startup_info.filepath = Some(path);
            }
        }
    }

    pub fn update(&mut self, ui: &mut Ui) {
        if !self.looked_in_args {
            self.looked_in_args = true;
            if let Some(filepath) = Self::get_filepath() {
                self.startup_info.filepath = Some(filepath);
                self.use_selected_filepath = true;
            }
        }

        ui.vertical_centered(|ui| {
            ui.add_space(10f32);

            ui.label("Please specify the path to the chip 8 program to execute.");

            ui.end_row();

            ui.label("Drag-and-drop the chip 8 program here, or specify the path using the file dialog.");

            ui.end_row();
            ui.add_space(20f32);

            self.file_dialog(ui);

            ui.end_row();
            ui.add_space(20f32);

            ui.label("Selected Path:");

            ui.end_row();

            if let Some(filepath) = &self.startup_info.filepath {
                ui.label(filepath.to_str().unwrap());
            } else {
                ui.label("No Filepath Selected");
            }

            ui.end_row();
            ui.add_space(30f32);

            if utils::button("Use selected Path", ui).clicked() {
                self.use_selected_filepath = true;
            }

            ui.end_row();
            ui.add_space(20f32)
        });

        self.collect_dropped_files(ui.ctx());
    }

    fn get_filepath() -> Option<PathBuf> {
        let args = env::args();

        if args.len() > 1 {
            let args: Vec<String> = args.collect();
            Some(PathBuf::from(args[1].to_owned()))
        } else {
            None
        }
    }
}


#[derive(Default)]
pub struct StartUpInfo {
    pub filepath: Option<PathBuf>,
}
