use crate::chip_8::emulator::Emulator;
use crate::ui::Screen;
use crate::ui::MENU_BAR_OFFSET;
use anyhow::Error;
use eframe::egui::{Context, FontId, ViewportCommand};
use eframe::{egui, Frame};
use std::cell::RefCell;
use std::cmp::PartialEq;
use std::rc::Rc;
use std::time::{Duration, Instant};

pub const FONT_SIZE: f32 = 20f32;

pub struct EmulatorApp {
    pub emulator: Rc<RefCell<Emulator>>,
    pub screen: Screen,
    pub state: AppState,
    pub frame_data: Rc<RefCell<FrameData>>,
}

impl Default for EmulatorApp {
    fn default() -> Self {
        let emulator = Rc::new(RefCell::new(Emulator::default()));
        let frame_data = Rc::new(RefCell::new(FrameData::default()));
        let screen = Screen::new(emulator.clone(), frame_data.clone());

        Self {
            emulator,
            screen,
            state: AppState::default(),
            frame_data,
        }
    }
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

    fn emulate(&mut self, ctx: &Context) -> Option<Event> {
        self.frame_data.borrow_mut().wait_for_next_frame();

        let emulator = &mut *self.emulator.borrow_mut();

        ctx.input(|input| {
            emulator.keypad.update_keys(input);
        });

        emulator.tick_timers();

        for _ in 0..emulator.config.instructions_per_frame {
            if let Err(event) = emulator.execute_instruction() {
                return Some(event);
            }
        }

        None
    }

    fn on_event(&mut self, event: Event, ctx: &Context) {
        match event {
            Event::StartEmulation => {
                self.state = AppState::Emulating;
                self.frame_data.borrow_mut().next_frame = Instant::now();
            },

            Event::PauseEmulation => self.state = AppState::Paused,
            Event::OpenSettings => self.state = AppState::Settings,
            Event::ReportError(error) => self.state = AppState::ErrorReporting(error),
            Event::Exit => ctx.send_viewport_cmd(ViewportCommand::Close),
        }
    }
}

impl eframe::App for EmulatorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ctx.request_repaint();

        let event = self
            .screen
            .draw_main_screen(ctx, self.state == AppState::Paused);

        if let Some(event) = event {
            self.on_event(event, ctx);
        }

        let event = match &mut self.state {
            AppState::Emulating => self.emulate(ctx),
            AppState::Settings => self.screen.draw_settings(ctx),
            AppState::ErrorReporting(error) => self.screen.draw_error(ctx, error),
            AppState::Paused => None,
        };

        if let Some(event) = event {
            self.on_event(event, ctx);
        }
    }
}

pub enum AppState {
    Emulating,
    Paused,
    Settings,
    ErrorReporting(Error),
}

impl Default for AppState {
    fn default() -> Self {
        Self::Settings
    }
}

impl PartialEq for AppState {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

pub enum Event {
    StartEmulation,
    PauseEmulation,
    OpenSettings,
    ReportError(Error),
    Exit,
}

pub struct FrameData {
    pub next_frame: Instant,
    pub sleep_time: Duration,
}

impl Default for FrameData {
    fn default() -> Self {
        Self {
            next_frame: Instant::now(),
            sleep_time: Duration::from_secs(0),
        }
    }
}

impl FrameData {
    pub fn wait_for_next_frame(&mut self) {
        self.next_frame += Duration::from_secs_f32(1f32 / 60f32);

        let sleep_time = self.next_frame - Instant::now();
        self.sleep_time = sleep_time;

        std::thread::sleep(sleep_time);
    }
}
