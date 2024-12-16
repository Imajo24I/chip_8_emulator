use crate::chip_8::emulator_window::{EmulatorWindow, MENU_BAR_OFFSET};
use crate::error_report_window::ErrorReportWindow;
use crate::events::Event;
use crate::startup::StartupWindow;
use eframe::egui::{Context, FontId};
use eframe::{egui, Frame};

pub const FONT_SIZE: f32 = 20f32;

#[derive(Default)]
pub struct EmulatorApp {
    pub state: AppState,
}

impl EmulatorApp {
    pub fn run() {
        eframe::run_native(
            "Chip 8 Emulator",
            Self::options(),
            Box::new(|cc| {
                cc.egui_ctx.style_mut(|style| {
                    style.override_font_id = Some(FontId::proportional(FONT_SIZE));
                });

                Ok(Box::new(Self::default()))
            }),
        )
        .unwrap();
    }

    fn options() -> eframe::NativeOptions {
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([1280f32, 640f32 + MENU_BAR_OFFSET])
                .with_icon(
                    eframe::icon_data::from_png_bytes(include_bytes!("../assets/icon.png"))
                        .expect("Failed to load icon."),
                ),
            ..Default::default()
        }
    }

    fn startup_window(ctx: &Context, window: &mut StartupWindow) -> Option<Event> {
        egui::Window::new("Startup")
            .default_size([840f32, 640f32])
            .collapsible(false)
            .show(ctx, |ui| window.update(ui))
            .unwrap()
            .inner
            .unwrap()
    }

    fn emulator_window(ctx: &Context, window: &mut EmulatorWindow) -> Option<Event> {
        egui::CentralPanel::default()
            .show(ctx, |ui| window.update(ui))
            .inner
    }

    fn error_report_window(ctx: &Context, window: &mut ErrorReportWindow) -> Option<Event> {
        egui::Window::new("Error executing Chip 8 Emulator")
            .collapsible(false)
            .default_size([830f32, 830f32])
            .show(ctx, |ui| window.update(ui))
            .unwrap()
            .inner
            .unwrap()
    }
}

impl eframe::App for EmulatorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let event = match &mut self.state {
            AppState::Emulating(window) => Self::emulator_window(ctx, window),
            AppState::Initializing(window) => Self::startup_window(ctx, window),
            AppState::ErrorReporting(window) => Self::error_report_window(ctx, window),
        };

        if let Some(event) = event {
            event.execute(ctx, self);
        }
    }
}

pub enum AppState {
    Initializing(StartupWindow),
    Emulating(EmulatorWindow),
    ErrorReporting(ErrorReportWindow),
}

impl Default for AppState {
    fn default() -> Self {
        Self::Initializing(StartupWindow::default())
    }
}
