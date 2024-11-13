use crate::errors::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

const FONT_BYTES: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Emulator {
    pub display: [[bool; 64]; 32],

    // Memory
    // 4096 bytes of memory
    memory: [u8; 4096],

    // Program Counter
    // Used to store location of the next instruction
    pc: usize,

    // Index Register
    // Used to point at locations in memory
    i_register: u16,

    // Stack
    // Used to call and return from subroutines (functions)
    stack: Vec<u16>,

    // General-purpose variable registers
    v_registers: [u8; 16],

    // Delay Timer
    // Decrements 60 times per second
    delay_timer: u8,

    // Sound Timer
    // Functions like the Delay Timer, however also gives of a beep sounds when not 0
    sound_timer: u8,
}

impl Emulator {
    pub fn new(filepath: &Path) -> Result<Self, Error> {
        Ok(Self {
            display: [[false; 64]; 32],
            // Start at 0x200, since 0x000 - 0x1FF are reserved for interpreter
            pc: 0x200,
            i_register: 0,
            stack: Vec::new(),
            v_registers: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            memory: Self::memory_from_file(filepath)?,
        })
    }

    fn memory_from_file(filepath: &Path) -> Result<[u8; 4096], Error> {
        let memory = [0; 4096];
        let mut file = File::open(filepath);

        match file {
            Ok(mut file) => {
                let mut data = vec![];
                let result = file.read_to_end(&mut data);

                if let Err(error) = result {
                    return Err(
                        Error::new_with_cause(
                            format!("Error reading file at {}. Please ensure it is a valid file.", filepath).as_str(),
                            Box::new(error),
                        )
                    );
                }

                // Add instructions and font to memory
                memory[0x200..0x200 + data.len()].copy_from_slice(&data);
                memory[0x050..0x0A0].copy_from_slice(&FONT_BYTES);
            }

            Err(error) => {
                return Err(
                    Error::new_with_cause(
                        format!("Error opening file at {}. Please ensure the path points to a valid file", filepath).as_str(),
                        Box::new(error),
                    )
                )
            }
        }

        Ok(memory)
    }
}