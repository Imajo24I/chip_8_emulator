use eframe::egui::{FontId, IconData, Response, RichText, Ui};
use eframe::CreationContext;

const FONT_SIZE: f32 = 20f32;

/// Returns the icon data for the applications icon
pub fn icon_data() -> IconData {
    eframe::icon_data::from_png_bytes(
        include_bytes!("../assets/icon.png")
    ).expect("Failed to load icon.")
}

/// Sets the default style for the windows
pub fn set_default_style(cc: &CreationContext) {
    cc.egui_ctx.style_mut(|style| {
        style.override_font_id = Some(FontId::proportional(FONT_SIZE));
    });
}

// Reason for this to exist, is that Style::override_font_id currently isn't respected
// in stuff like Buttons, ComboBoxes, etc.
// Has been fixed, however there's no release with the fix yet.
// https://github.com/emilk/egui/pull/5310
// TODO: Remove this once the fix is released
/// Create a button with font size of 20
pub fn button(text: &str, ui: &mut Ui) -> Response {
    ui.button(RichText::new(text).font(FontId::proportional(FONT_SIZE)))
}
