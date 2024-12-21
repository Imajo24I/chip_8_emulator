use crate::chip_8::config::Config;
use eframe::egui::{Align, TextEdit, Ui};

pub fn draw_settings(ui: &mut Ui, config: &mut Config) {
    ui.checkbox(
        &mut config.use_german_keyboard_layout,
        "Use german keyboard layout",
    );

    ui.end_row();
    ui.add_space(20f32);

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
}
