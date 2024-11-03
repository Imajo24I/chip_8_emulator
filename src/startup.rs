use crate::utils::icon_data;
use eframe::egui::{Context, FontId, RichText, Ui, ViewportCommand};
use eframe::{egui, App, Frame};
use std::env;

pub fn get_filepath() -> String {
    let args = env::args();

    if args.len() > 1 {
        let args: Vec<String> = args.collect();
        args[1].to_owned()
    } else {
        run_startup_window().filepath
    }
}

fn run_startup_window() -> StartUpInfo {
    let mut startup_info = StartUpInfo::default();

    eframe::run_native(
        "Chip 8 Emulator - Startup Manager",
        StartupWindow::options(),
        Box::new(|_cc| {
            Ok(Box::<StartupWindow>::new(
                StartupWindow::new(&mut startup_info)
            ))
        }),
    ).unwrap();

    startup_info
}

struct StartupWindow<'a> {
    startup_info: &'a mut StartUpInfo,
}

impl<'a> StartupWindow<'a> {
    fn new(startup_info: &'a mut StartUpInfo) -> Self {
        Self {
            startup_info,
        }
    }

    pub fn options() -> eframe::NativeOptions {
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([840f32, 530f32])
                .with_icon(icon_data()),
            ..Default::default()
        }
    }

    fn default_text(text: &str) -> RichText {
        RichText::new(text).font(FontId::proportional(20f32))
    }

    fn default_label(text: &str, ui: &mut Ui) {
        ui.label(Self::default_text(text));
    }

    fn collect_dropped_files(&mut self, ctx: &Context) {
        ctx.input(|input| {
            if !input.raw.dropped_files.is_empty() {
                self.startup_info.filepath = input.raw.dropped_files[0]
                    .path.clone().unwrap().into_os_string().into_string().unwrap();
            }
        });
    }

    fn file_dialog(&mut self, ui: &mut Ui) {
        if ui.button(RichText::new("Open File...")
            .font(FontId::proportional(20f32)))
            .clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                self.startup_info.filepath = path.into_os_string().into_string().unwrap();
            }
        }
    }
}

impl App for StartupWindow<'_> {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(RichText::new("Startup Manager")
                    .font(FontId::proportional(40f32)));

                ui.separator();
                ui.add_space(60f32);

                Self::default_label("Please specify the path to the chip 8 program to execute.", ui);

                ui.end_row();

                Self::default_label("Drag-and-drop the chip 8 program here, or specify the path using the file dialog.", ui);

                ui.end_row();
                ui.add_space(20f32);

                self.file_dialog(ui);

                ui.end_row();
                ui.add_space(20f32);

                Self::default_label("Selected Path:", ui);

                ui.end_row();

                Self::default_label(self.startup_info.filepath.as_str(), ui);

                ui.end_row();
                ui.add_space(30f32);

                if ui.button(Self::default_text("Use selected Path")).clicked() {
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }
            });
        });

        self.collect_dropped_files(ctx);
    }
}

struct StartUpInfo {
    filepath: String,
}

impl Default for StartUpInfo {
    fn default() -> Self {
        Self {
            filepath: "---".to_string(),
        }
    }
}
