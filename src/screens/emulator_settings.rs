use crate::chip_8::config::Config;
use eframe::egui::{Align, TextEdit, Ui};

pub fn draw_settings(ui: &mut Ui, config: &mut Config) {
    ui.collapsing("Emulation Speed", |ui| {
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
    });

    ui.add_space(10f32);

    ui.collapsing("Emulation Quirks", |ui| {
        let quirks = &mut config.quirks;

        ui.checkbox(&mut quirks.vf_reset, "Reset VF Register");
        ui.checkbox(&mut quirks.increment_i_reg, "Increment I Register");
        ui.checkbox(&mut quirks.vx_offset_jump, "Use VX as offset");
        ui.checkbox(&mut quirks.shift_vx_directly, "Shift VX directly");
        ui.checkbox(&mut quirks.wrap_sprites, "Wrap Sprites");
    });

    ui.add_space(10f32);

    ui.collapsing("Other Settings", |ui| {
        ui.checkbox(
            &mut config.use_german_keyboard_layout,
            "Use german keyboard layout",
        );
    });
}
