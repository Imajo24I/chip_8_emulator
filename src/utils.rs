use eframe::egui::IconData;

pub fn icon_data() -> IconData {
    eframe::icon_data::from_png_bytes(
            include_bytes!("../assets/icon.png")
        ).expect("Failed to load icon.")
}