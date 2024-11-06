use std::path::PathBuf;

pub struct Chip8Emulator {
    pub display: [[bool; 64]; 32]
}

impl Chip8Emulator {
    pub fn new(filepath: PathBuf) -> Self {
        Self {
            display: [[false; 64]; 32]
        }
    }
}