use eframe::egui::{FontId, Response, RichText, Ui};

// Reason for this to exist, is that Style::override_font_id currently isn't respected
// in stuff like Buttons, ComboBoxes, etc.
// Has been fixed, however there's no release with the fix yet.
// https://github.com/emilk/egui/pull/5310
// TODO: Remove this once the fix is released
/// Create a button with font size of 20
pub fn button(text: &str, ui: &mut Ui) -> Response {
    ui.button(RichText::new(text).font(FontId::proportional(crate::emulator_app::FONT_SIZE)))
}
