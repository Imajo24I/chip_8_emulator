use crate::chip_8::emulator::Emulator;
use crate::emulator_app::{Event, FrameData};
use crate::ui::{MENU_BAR_OFFSET, TEXTURE_OPTIONS};
use eframe::egui;
use eframe::egui::{Ui, Vec2};
use std::cell::RefCell;
use std::rc::Rc;

pub struct MainScreen {
    emulator: Rc<RefCell<Emulator>>,
    frame_data: Rc<RefCell<FrameData>>,
}

impl MainScreen {
    pub fn new(emulator: Rc<RefCell<Emulator>>, frame_data: Rc<RefCell<FrameData>>) -> Self {
        Self {
            emulator,
            frame_data,
        }
    }

    pub fn draw_main_screen(
        &mut self,
        ui: &mut Ui,
        window_size: Vec2,
        paused: bool,
    ) -> Option<Event> {
        self.draw_display(ui, window_size);
        self.draw_menu_bar(ui, window_size, paused)
    }

    fn draw_display(&mut self, ui: &mut Ui, window_size: Vec2) {
        let emulator = self.emulator.borrow();

        let width = emulator.display.resolution.width();
        let height = emulator.display.resolution.height();

        let mut image_data: Vec<u8> = Vec::with_capacity(width * height * 4);

        for row in emulator.display.zip_planes() {
            for pixel in row {
                let color = match pixel {
                    (false, false) => egui::Color32::BLACK,
                    (true, false) => egui::Color32::WHITE,
                    (false, true) => egui::Color32::LIGHT_GREEN,
                    (true, true) => egui::Color32::DARK_GREEN,
                };

                image_data.extend_from_slice(&[color.r(), color.g(), color.b(), color.a()]);
            }
        }

        let color_image = egui::ColorImage::from_rgba_unmultiplied([width, height], &image_data);
        let texture_handle = ui
            .ctx()
            .load_texture("display_texture", color_image, TEXTURE_OPTIONS);

        egui::Image::from(&texture_handle).paint_at(
            ui,
            egui::Rect::from_two_pos(
                egui::Pos2::ZERO,
                egui::Pos2::new(window_size.x, window_size.y - MENU_BAR_OFFSET),
            ),
        );
    }

    fn draw_menu_bar(&mut self, ui: &mut Ui, window_size: Vec2, paused: bool) -> Option<Event> {
        let emulator = self.emulator.borrow();

        let bar_height = window_size.y - MENU_BAR_OFFSET + 5f32;
        let bar_top_height = window_size.y - 5f32;
        let window_center = window_size.x / 2f32;

        if ui
            .put(
                egui::Rect::from_two_pos(
                    egui::Pos2::new(window_center - 50f32, bar_height),
                    egui::Pos2::new(window_center + 50f32, bar_top_height),
                ),
                egui::Button::new(egui::RichText::new("Open Settings")),
            )
            .clicked()
        {
            emulator.beeper.pause();
            return Some(Event::OpenSettings);
        }

        if ui
            .put(
                egui::Rect::from_two_pos(
                    egui::Pos2::new(window_center - 200f32, bar_height),
                    egui::Pos2::new(window_center - 100f32, bar_top_height),
                ),
                egui::Button::new("Pause/Resume"),
            )
            .clicked()
        {
            return if paused {
                Some(Event::StartEmulation)
            } else {
                Some(Event::PauseEmulation)
            };
        }

        if ui
            .put(
                egui::Rect::from_two_pos(
                    egui::Pos2::new(window_center + 100f32, bar_height),
                    egui::Pos2::new(window_center + 200f32, bar_top_height),
                ),
                egui::Button::new("Exit Emulator"),
            )
            .clicked()
        {
            return Some(Event::Exit);
        }

        ui.put(
            egui::Rect::from_two_pos(
                egui::Pos2::new(100f32, bar_height - 2f32),
                egui::Pos2::new(300f32, bar_top_height),
            ),
            egui::Label::new(if paused {
                "Emulation Paused"
            } else {
                "Emulation Running"
            }),
        );

        let frame_time = ui.ctx().input(|input| input.stable_dt * 1000f32)
            - self.frame_data.borrow().sleep_time.as_millis() as f32;

        ui.put(
            egui::Rect::from_two_pos(
                egui::Pos2::new(window_size.x - 400f32, bar_height - 2f32),
                egui::Pos2::new(window_size.x - 100f32, bar_top_height),
            ),
            egui::Label::new(format!("Frame Time: {:.2}ms", frame_time)),
        )
        .on_hover_text(
            "The frame time is how long a full frame takes, without considering the FPS limiting.",
        );

        None
    }
}
