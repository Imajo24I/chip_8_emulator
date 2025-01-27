use crate::chip_8::config::Quirks;
use crate::chip_8::emulator::Emulator;
use crate::chip_8::keypad::{Key, HEX_KEYS};
use eframe::egui::{ComboBox, Id, Slider, SliderClamping, Ui, Widget};
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
        )
        .on_hover_text("How many Instructions are executed every frame. The target FPS is 60.");

        ui.add_space(5f32);

        let memory_size = config.memory_size;

        ComboBox::from_label("Memory Size")
            .selected_text(config.memory_size.to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut config.memory_size, 4096, "4096 (Chip 8 & SuperChip)");
                ui.selectable_value(&mut config.memory_size, 65536, "65536 (XO-Chip)");
            })
            .response
            .on_hover_text(
                "Choose between 4096 (Chip 8 & SuperChip) and 65536 (XO-Chip) bytes of memory.",
            );

        if memory_size != config.memory_size {
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
    })
    .header_response
    .on_hover_text(
        "Different Chip 8 programs require different emulation quirks. Configure these here",
    );
}

fn draw_keybindings(ui: &mut Ui, keys: &mut [Key; 16]) {
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

fn draw_other_settings(ui: &mut Ui, emulator: &mut Emulator) {
    ui.collapsing("Other Settings", |ui| {
        let mut volume = emulator.beeper.get_volume();

        if ui
            .add(Slider::new(&mut volume, 0f32..=1f32).text("Beeper Volume"))
            .changed()
        {
            emulator.beeper.set_volume(volume);
        }
    });
}
