use crate::chip_8::emulator::Emulator;
use crate::chip_8::keypad::HEX_KEYS;
use crate::emulator_app::Event;
use eframe::egui::{ComboBox, Context, Id, Slider, SliderClamping, Ui, Widget};
use egui_keybind::Keybind;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub struct Settings {
    emulator: Rc<RefCell<Emulator>>,
}

impl Settings {
    pub fn new(emulator: Rc<RefCell<Emulator>>) -> Self {
        Self { emulator }
    }

    pub fn draw_settings(&self, ui: &mut Ui) -> Option<Event> {
        ui.vertical_centered(|ui| {
            if let Some(filepath) = &self.emulator.borrow().get_rom() {
                ui.label(format!(
                    "Selected ROM: {}",
                    filepath.file_name().unwrap().to_string_lossy()
                ));
            } else {
                ui.label("No ROM selected");
            }

            ui.add_space(5.0);

            self.collect_dropped_files(ui.ctx());
            self.file_dialog(ui);
            ui.add_space(5.0);

            if self.emulator.borrow().rom_loaded && ui.button("Reload ROM").clicked() {
                let emulator = &mut *self.emulator.borrow_mut();

                emulator.reset();
                if let Err(error) = emulator.load_rom() {
                    return Some(Event::ReportError(error));
                }
            }

            ui.add_space(15.0);

            self.draw_emulation_settings(ui);
            ui.add_space(10.0);

            self.draw_emulation_quirks(ui);
            ui.add_space(10.0);

            self.draw_keybindings(ui);
            ui.add_space(10.0);

            self.draw_other_settings(ui);
            ui.add_space(15.0);

            if ui.button("Start/Resume Emulation").clicked() {
                let emulator = &mut *self.emulator.borrow_mut();

                if emulator.get_rom().is_some() {
                    if !emulator.rom_loaded {
                        let result = emulator.load_rom();

                        if let Err(error) = result {
                            return Some(Event::ReportError(error));
                        }
                    }

                    return Some(Event::StartEmulation);
                }
            }

            None
        })
        .inner
    }

    fn collect_dropped_files(&self, ctx: &Context) {
        ctx.input(|input| {
            if !input.raw.dropped_files.is_empty() {
                let filepath = input.raw.dropped_files[0].path.clone().unwrap();
                self.select_rom(filepath);
            }
        })
    }

    fn file_dialog(&self, ui: &mut Ui) {
        if ui.button("Select File...").clicked() {
            if let Some(filepath) = rfd::FileDialog::new().pick_file() {
                self.select_rom(filepath);
            }
        }
    }

    fn select_rom(&self, filepath: PathBuf) {
        let emulator = &mut *self.emulator.borrow_mut();

        if emulator.rom_loaded {
            emulator.reset();
        }

        emulator.select_rom(filepath);
    }

    fn draw_emulation_settings(&self, ui: &mut Ui) {
        let emulator = &mut *self.emulator.borrow_mut();

        ui.collapsing("Emulation Settings", |ui| {
            let memory = &mut emulator.memory;

            ui.add(
                Slider::new(&mut emulator.config.instructions_per_frame, 0..=1000)
                    .clamping(SliderClamping::Never)
                    .text("Instructions per Frame"),
            )
            .on_hover_text("How many Instructions are executed every frame. The target FPS is 60.");

            ui.add_space(5f32);

            let memory_size = memory.size;

            ComboBox::from_label("Memory Size")
                .selected_text(memory.size.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut memory.size, 4096, "4096 (Chip 8 & SuperChip)");
                    ui.selectable_value(&mut memory.size, 65536, "65536 (XO-Chip)");
                })
                .response
                .on_hover_text(
                    "Choose between 4096 (Chip 8 & SuperChip) and 65536 (XO-Chip) bytes of memory.",
                );

            if memory_size != memory.size {
                memory.resize(memory.size);
            }
        });
    }

    fn draw_emulation_quirks(&self, ui: &mut Ui) {
        let quirks = &mut self.emulator.borrow_mut().config.quirks;

        ui.collapsing("Emulation Quirks", |ui| {
            ui.checkbox(&mut quirks.vf_reset, "Reset VF Register");
            ui.checkbox(&mut quirks.increment_i_reg, "Increment I Register");
            ui.checkbox(&mut quirks.vx_offset_jump, "Use VX as offset");
            ui.checkbox(&mut quirks.shift_vx_directly, "Shift VX directly");
            ui.checkbox(&mut quirks.wrap_sprites, "Wrap Sprites");
        })
        .header_response
        .on_hover_text(
            "Different Chip 8 programs require different emulation quirks. Configure these here",
        );
    }

    fn draw_keybindings(&self, ui: &mut Ui) {
        let keys = &mut self.emulator.borrow_mut().keypad.keys;

        ui.collapsing("Keybindings", |ui| {
            for row in 0..4 {
                ui.horizontal(|ui| {
                    for key_index in 0..4 {
                        let key = &mut keys[HEX_KEYS[row * 4 + key_index] as usize];

                        Keybind::new(&mut key.egui_key, Id::from(key.hex_key.to_string()))
                            .ui(ui)
                            .on_hover_text(format!("Chip 8 key: {:1X}", key.hex_key));
                    }
                });
            }
        })
        .header_response
        .on_hover_text("Configure keybindings for the Chip 8 Keypad here.");
    }

    fn draw_other_settings(&self, ui: &mut Ui) {
        let beeper = &mut self.emulator.borrow_mut().beeper;

        ui.collapsing("Other Settings", |ui| {
            let mut volume = beeper.get_volume();

            if ui
                .add(Slider::new(&mut volume, 0f32..=1f32).text("Beeper Volume"))
                .changed()
            {
                beeper.set_volume(volume);
            }
        });
    }
}
