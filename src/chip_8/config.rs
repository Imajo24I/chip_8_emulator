#[derive(Copy, Clone)]
pub struct Config {
    pub cycles_per_frame: u32,
    pub use_german_keyboard_layout: bool,
    pub emulation_paused: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cycles_per_frame: 10,
            use_german_keyboard_layout: true,
            emulation_paused: false,
        }
    }
}
