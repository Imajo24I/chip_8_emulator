use crate::chip_8::beep::Beeper;
use crate::chip_8::config::Quirks;
use crate::chip_8::emulator::Emulator;
use eframe::egui::{Align, Slider, TextEdit, Ui};

pub fn draw_settings(ui: &mut Ui, emulator: &mut Emulator) {
    draw_emulation_settings(ui, emulator);
    ui.add_space(10f32);

    draw_emulation_quirks(ui, &mut emulator.config.quirks);
    ui.add_space(10f32);

    draw_other_settings(ui, emulator);
}

fn draw_emulation_settings(ui: &mut Ui, emulator: &mut Emulator) {
    let config = &mut emulator.config;

    ui.collapsing("Emulation Settings", |ui| {
        // TODO: Rename this and usages to instructions instead of cycles
        // TODO: Use Built-In Slider instead?
        ui.label("Cycles per Frame:");
        ui.add_space(5f32);

        let cycles_per_frame = &mut config.cycles_per_frame;
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

        ui.add_space(5f32);

        if ui.button(format!("Memory Size: {}", config.memory_size)).clicked() {
            config.memory_size = if config.memory_size == 4096 { 65536 } else { 4096 };
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

        ui.add_space(5.0);

        if ui
            .checkbox(
                &mut emulator.config.use_german_keyboard_layout,
                "Use german keyboard layout",
            )
            .clicked()
        {
            emulator
                .keypad
                .update_layout(emulator.config.use_german_keyboard_layout);
        }
    });
}
