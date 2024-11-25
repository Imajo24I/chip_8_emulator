use crate::chip_8::instructions;
use crate::errors::error::{Cause, Error};
use crate::events::Event;

use crate::chip_8::keypad::Keypad;
use std::fs::File;
use std::io::Read;
use std::path::Path;

const FONT_SET: [u8; 80] = [
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

const MEMORY_SIZE: usize = 4096;
// Instructions start at 0x200, since 0x000 - 0x1FF are reserved for interpreter
const INSTRUCTIONS_START: usize = 0x200;

pub struct Emulator {
    pub display: [[bool; 64]; 32],

    pub keypad: Keypad,

    // Memory
    // 4096 bytes of memory
    pub memory: [u8; MEMORY_SIZE],

    // Program Counter
    // Used to store location of the next instruction
    pub pc: usize,

    // Index Register
    // Used to point at locations in memory
    pub i_register: usize,

    // Stack
    // Used to call and return from subroutines (functions)
    pub stack: Vec<usize>,

    // General-purpose variable registers
    pub v_registers: [u8; 16],

    // Delay Timer
    // Decrements 60 times per second
    pub delay_timer: u8,

    // Sound Timer
    // Functions like the Delay Timer, however also gives of a beep sounds when not 0
    pub sound_timer: u8,
}

impl Emulator {
    pub fn new(filepath: &Path) -> Result<Self, Error> {
        Ok(Self {
            display: [[false; 64]; 32],
            keypad: Keypad::default(),
            pc: INSTRUCTIONS_START,
            i_register: 0,
            stack: Vec::new(),
            v_registers: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            memory: Self::memory_from_file(filepath)?,
        })
    }

    fn memory_from_file(filepath: &Path) -> Result<[u8; MEMORY_SIZE], Error> {
        let mut memory = [0; MEMORY_SIZE];
        let file = File::open(filepath);

        match file {
            Ok(mut file) => {
                let mut data = vec![];
                let result = file.read_to_end(&mut data);

                if let Err(error) = result {
                    return Err(
                        Error::new(
                            format!("Error reading file at {} - Please ensure it is a valid file.", filepath.display()),
                            Cause::new(None, Some(Box::new(error))),
                        ),
                    );
                }

                if data.len() > MEMORY_SIZE - INSTRUCTIONS_START {
                    return Err(Error::new(
                        format!("Error reading file at {} - File exceeds maximum data size of {} bytes.", filepath.display(), MEMORY_SIZE - INSTRUCTIONS_START),
                        Cause::new(Some(format!("File with size of {} bytes exceeds maximum data size of {} bytes.", data.len(), MEMORY_SIZE - INSTRUCTIONS_START)), None),
                    ));
                }

                // Add instructions and font to memory
                memory[INSTRUCTIONS_START..INSTRUCTIONS_START + data.len()].copy_from_slice(&data);
                memory[0..FONT_SET.len()].copy_from_slice(&FONT_SET);
            }

            Err(error) => {
                return Err(
                    Error::new(
                        format!("Error opening file at {} - Please ensure the path points to a valid file", filepath.display()),
                        Cause::new(None, Some(Box::new(error))),
                    ),
                )
            }
        }

        Ok(memory)
    }

    pub fn run_cycle(&mut self) -> Result<(), Event> {
        // Exit if no more instructions left
        if self.pc >= MEMORY_SIZE {
            return Err(Event::Exit);
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
            self.make_sound();
        }

        // fetch opcode
        let opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
        self.pc += 2;

        instructions::execute_instruction(self, opcode)
    }

    fn make_sound(&mut self) {
        // TODO: Implement this
    }
}
