use crate::chip_8::emulator::Emulator;
use crate::screens::emulator_screen::{EmulatorScreen, MENU_BAR_OFFSET};
use crate::screens::error_report_screen::ErrorReportScreen;
use crate::screens::startup_screen::StartupScreen;
use anyhow::Error;
use eframe::egui::{Context, FontId, ViewportCommand};
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

    fn on_event(&mut self, event: Event, ctx: &Context) {
        match event {
            Event::StartEmulation(emulator) => {
                ctx.style_mut(|style| {
                    style.override_font_id = None;
                });

                let emulator_screen = EmulatorScreen::new(emulator);
                self.state = AppState::Emulating(emulator_screen);
            }

            Event::ReportError(error) => {
                ctx.style_mut(|style| {
                    style.override_font_id = Some(FontId::proportional(FONT_SIZE));
                });

                self.state = AppState::ErrorReporting(ErrorReportScreen::new(error))
            }

            Event::Exit => ctx.send_viewport_cmd(ViewportCommand::Close),
        }
    }

    fn draw_startup_screen(ctx: &Context, screen: &mut StartupScreen) -> Option<Event> {
        egui::Window::new("Startup")
            .default_size([440f32, 320f32])
            .collapsible(false)
            .show(ctx, |ui| screen.update(ui))
            .unwrap()
            .inner
            .unwrap()
    }

    fn draw_emulator_screen(ctx: &Context, screen: &mut EmulatorScreen) -> Option<Event> {
        egui::CentralPanel::default()
            .show(ctx, |ui| screen.update(ui))
            .inner
    }

    fn draw_error_report_screen(ctx: &Context, screen: &mut ErrorReportScreen) -> Option<Event> {
        egui::Window::new("Error executing Chip 8 Emulator")
            .collapsible(false)
            .default_size([830f32, 830f32])
            .show(ctx, |ui| screen.update(ui))
            .unwrap()
            .inner
            .unwrap()
    }
}

impl eframe::App for EmulatorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let event = match &mut self.state {
            AppState::Emulating(screen) => Self::draw_emulator_screen(ctx, screen),
            AppState::Initializing(screen) => Self::draw_startup_screen(ctx, screen),
            AppState::ErrorReporting(screen) => Self::draw_error_report_screen(ctx, screen),
        };

        if let Some(event) = event {
            self.on_event(event, ctx);
        }
    }
}

pub enum AppState {
    Initializing(StartupScreen),
    Emulating(EmulatorScreen),
    ErrorReporting(ErrorReportScreen),
}

impl Default for AppState {
    fn default() -> Self {
        Self::Initializing(StartupScreen::default())
    }
}

pub enum Event {
    StartEmulation(Emulator),
    ReportError(Error),
    Exit,
}
