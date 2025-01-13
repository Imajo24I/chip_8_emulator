use crate::chip_8::beep::Beeper;
use crate::chip_8::config::Quirks;
use crate::chip_8::emulator::Emulator;
use crate::chip_8::keypad::{Key, HEX_KEYS};
use eframe::egui::{Id, Slider, SliderClamping, Ui, Widget};
use egui_keybind::Keybind;

pub fn draw_settings(ui: &mut Ui, emulator: &mut Emulator) {
    draw_emulation_settings(ui, emulator);
    ui.add_space(10f32);

    draw_emulation_quirks(ui, &mut emulator.config.quirks);
    ui.add_space(10f32);

    draw_keybindings(ui, &mut emulator.keypad.keys);
    ui.add_space(10f32);

    draw_other_settings(ui, emulator);
}

fn draw_emulation_settings(ui: &mut Ui, emulator: &mut Emulator) {
    let config = &mut emulator.config;

    ui.collapsing("Emulation Settings", |ui| {
        ui.add(
            Slider::new(&mut config.instructions_per_frame, 0..=1000)
                .clamping(SliderClamping::Never)
                .text("Instructions per Frame"),
        );

        ui.add_space(5f32);

        if ui
            .button(format!("Memory Size: {}", config.memory_size))
            .clicked()
        {
            config.memory_size = if config.memory_size == 4096 {
                65536
            } else {
                4096
            };
            emulator.memory.resize(config.memory_size, 0);
        }
    });
}

fn draw_emulation_quirks(ui: &mut Ui, quirks: &mut Quirks) {
    ui.collapsing("Emulation Quirks", |ui| {
        ui.checkbox(&mut quirks.vf_reset, "Reset VF Register");
        ui.checkbox(&mut quirks.increment_i_reg, "Increment I Register");
        ui.checkbox(&mut quirks.vx_offset_jump, "Use VX as offset");
        ui.checkbox(&mut quirks.shift_vx_directly, "Shift VX directly");
        ui.checkbox(&mut quirks.wrap_sprites, "Wrap Sprites");
    });
}

fn draw_keybindings(ui: &mut Ui, keys: &mut [Key; 16]) {
    ui.collapsing("Keybindings", |ui| {
        for row in 0..4 {
            ui.horizontal(|ui| {
                for key_index in 0..4 {
                    let key = &mut keys[HEX_KEYS[row * 4 + key_index] as usize];
                    Keybind::new(&mut key.egui_key, Id::from(key.hex_key.to_string())).ui(ui);
                }
            });
        }
    });
}

fn draw_other_settings(ui: &mut Ui, emulator: &mut Emulator) {
    ui.collapsing("Other Settings", |ui| {
        if ui
            .add(
                Slider::new(&mut emulator.beeper.settings.volume, 0f32..=1f32)
                    .text("Beeper Volume"),
            )
            .changed()
        {
            let is_playing = emulator.beeper.is_playing;

            emulator.beeper = Beeper::new(emulator.beeper.settings);
            if is_playing {
                emulator.beeper.play();
            }
        }
    });
}
