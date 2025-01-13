use eframe::egui;
use std::array::from_fn;
use std::cmp::PartialEq;
use std::collections::HashMap;

pub const HEX_KEYS: [u8; 16] = [1, 2, 3, 0xC, 4, 5, 6, 0xD, 7, 8, 9, 0xE, 0xA, 0, 0xB, 0xF];

#[derive(Clone)]
pub struct Keypad {
    pub keys: [Key; 16],
}

impl Keypad {
    pub fn new() -> Self {
        let mappings = Self::default_key_mappings();
        let keys: [Key; 16] = from_fn(|i| Key::from_hex(i as u8, &mappings));

        Self { keys }
    }

    pub fn update_keys(&mut self, input_state: &egui::InputState) {
        for key in self.keys.iter_mut() {
            key.state = if input_state.key_down(key.egui_key) {
                KeyState::Pressed
            } else if input_state.key_released(key.egui_key) {
                KeyState::Released
            } else {
                KeyState::Unpressed
            }
        }
    }

    pub fn is_key_pressed(&self, key: usize) -> bool {
        self.keys[key].state == KeyState::Pressed
    }

    pub fn is_key_released(&self, key: usize) -> bool {
        self.keys[key].state == KeyState::Released
    }

    pub fn get_released_key(&self) -> Option<u8> {
        for (key, key_state) in self.keys.iter().enumerate() {
            if key_state.state == KeyState::Released {
                return Some(key as u8);
            }
        }

        None
    }

    pub fn default_key_mappings() -> HashMap<u8, egui::Key> {
        HashMap::from([
            (1, egui::Key::Num1),
            (2, egui::Key::Num2),
            (3, egui::Key::Num3),
            (0xC, egui::Key::Num4),
            (4, egui::Key::Q),
            (5, egui::Key::W),
            (6, egui::Key::E),
            (0xD, egui::Key::R),
            (7, egui::Key::A),
            (8, egui::Key::S),
            (9, egui::Key::D),
            (0xE, egui::Key::F),
            (0xA, egui::Key::Y),
            (0, egui::Key::X),
            (0xB, egui::Key::C),
            (0xF, egui::Key::V),
        ])
    }
}

#[derive(Clone)]
pub struct Key {
    pub state: KeyState,
    pub egui_key: egui::Key,
    pub hex_key: u8,
}

impl Key {
    pub fn from_hex(hex_key: u8, mapping: &HashMap<u8, egui::Key>) -> Self {
        Self {
            state: Default::default(),
            egui_key: mapping.get(&hex_key).cloned().unwrap(),
            hex_key,
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum KeyState {
    Unpressed,
    Pressed,
    Released,
}

impl Default for KeyState {
    fn default() -> Self {
        Self::Unpressed
    }
}
