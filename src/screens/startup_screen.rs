use crate::chip_8::config::Config;
use crate::events::Event;
use crate::screens::emulator_settings::draw_settings;
use eframe::egui::{Context, Ui};
use std::path::PathBuf;

#[derive(Default)]
pub struct StartupScreen {
    pub filepath: Option<PathBuf>,
    pub config: Config,
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

            draw_settings(ui, &mut self.config);

            ui.end_row();
            ui.add_space(20f32);

            if ui.button("Start Emulation").clicked() && self.filepath.is_some() {
                return Some(Event::StartEmulation(
                    self.filepath.clone().unwrap(),
                    self.config,
                ));
            }

            ui.end_row();
            ui.add_space(20f32);

            None
        })
        .inner
    }

    fn update_filepath(&mut self, ui: &mut Ui) {
        ui.add_space(10f32);

        ui.label("Selected File:");

        if let Some(filepath) = &self.filepath {
            ui.label(filepath.file_name().unwrap().to_string_lossy());
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
}
