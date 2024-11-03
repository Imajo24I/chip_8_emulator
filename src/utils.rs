use eframe::egui::{FontId, IconData, RichText, Ui};

pub fn icon_data() -> IconData {
    eframe::icon_data::from_png_bytes(
        include_bytes!("../assets/icon.png")
    ).expect("Failed to load icon.")
}


/// Returns RichText with the font size of 20, which is used the most in this project
pub fn richtext(text: &str) -> RichText {
    RichText::new(text).font(FontId::proportional(20f32))
}

/// Adds a label to the UI, with the font size of 20
pub fn label_from_str(text: &str, ui: &mut Ui) {
    ui.label(richtext(text));
}

pub fn label_from_string(text: String, ui: &mut Ui) {
    ui.label(richtext(&*text));
}