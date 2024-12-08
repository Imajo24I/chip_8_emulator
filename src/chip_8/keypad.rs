use crate::chip_8::emulator::Emulator;
use anyhow::{anyhow, Result};
use eframe::egui;
use std::array::from_fn;
use std::cmp::PartialEq;
use std::collections::HashMap;

pub struct Keypad {
    pub keys: [Key; 16],
}

impl Keypad {
    pub fn new(use_german_keyboard_layout: bool) -> Self {
        Self {
            keys: from_fn(|hex| Key::from_hex(hex as u8, use_german_keyboard_layout)),
        }
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

    pub fn is_key_valid(key: u8, opcode: u16, emulator: &Emulator) -> Result<()> {
        if key > 0xF {
            Err(
                anyhow!("Invalid instruction parameters - No key named {:#06x} exists - Instruction {:#06x} is located at memory location {}", key, opcode, emulator.pc - 2)
            )
        } else {
            Ok(())
        }
    }
}

pub struct Key {
    pub state: KeyState,
    pub egui_key: egui::Key,
    pub hey_key: u8,
}

impl Key {
    fn hex_to_key_hashmap(german_keyboard_layout: bool) -> HashMap<u8, egui::Key> {
        let a_key = if german_keyboard_layout {
            (0xA, egui::Key::Y)
        } else {
            (0xA, egui::Key::Z)
        };

        HashMap::from([
            (1, egui::Key::Num1),
            (2, egui::Key::Num2),
            (3, egui::Key::Num3),
            (0xC, egui::Key::Num4),
            (0x4, egui::Key::Q),
            (0x5, egui::Key::W),
            (0x6, egui::Key::E),
            (0xD, egui::Key::R),
            (0x7, egui::Key::A),
            (0x8, egui::Key::S),
            (0x9, egui::Key::D),
            (0xE, egui::Key::F),
            a_key,
            (0x0, egui::Key::X),
            (0xB, egui::Key::C),
            (0xF, egui::Key::V),
        ])
    }

    pub fn from_hex(hex_key: u8, use_german_keyboard: bool) -> Self {
        Self {
            state: Default::default(),
            egui_key: Self::hex_to_key_hashmap(use_german_keyboard)
                .get(&hex_key)
                .cloned()
                .unwrap(),
            hey_key: hex_key,
        }
    }
}

#[derive(PartialEq)]
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
