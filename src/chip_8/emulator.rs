use crate::chip_8::instructions;
use crate::events::Event;

use crate::chip_8::beep::Beeper;
use crate::chip_8::keypad::Keypad;
use anyhow::{anyhow, Error};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Duration;

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
    pub config: Config,

    pub beeper: Beeper,

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
    pub fn new(filepath: &Path, config: Config) -> Result<Self, Error> {
        Ok(Self {
            config,
            beeper: Beeper::new(),
            display: [[false; 64]; 32],
            keypad: Keypad::new(config.use_german_keyboard_layout),
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
                    return Err(anyhow!(error).context(format!(
                        "Error reading file at {}\nPlease ensure it is a valid file.",
                        filepath.display()
                    )));
                }

                if data.len() > MEMORY_SIZE - INSTRUCTIONS_START {
                    return Err(anyhow!(
                        "File with size of {} bytes exceeds maximum data size of {} bytes.",
                        data.len(),
                        MEMORY_SIZE - INSTRUCTIONS_START
                    ));
                }

                // Add instructions and font to memory
                memory[INSTRUCTIONS_START..INSTRUCTIONS_START + data.len()].copy_from_slice(&data);
                memory[0..FONT_SET.len()].copy_from_slice(&FONT_SET);
            }

            Err(error) => {
                return Err(anyhow!(error).context(format!(
                    "Error opening file at {}\nPlease ensure the path points to a valid file",
                    filepath.display()
                )))
            }
        }

        Ok(memory)
    }

    pub fn run_cycle(&mut self) -> Result<(), Event> {
        // Exit if no more instructions left
        if self.pc >= MEMORY_SIZE {
            self.beeper.stop();
            return Err(Event::Exit);
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;

            if self.sound_timer == 0 {
                self.beeper.pause();
            }
        }

        // fetch opcode
        let opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
        self.pc += 2;

        if let Err(error) = instructions::execute_instruction(self, opcode) {
            Err(Event::ReportError(error))
        } else {
            Ok(())
        }
    }
}

#[derive(Copy, Clone)]
pub struct Config {
    pub cycle_time: Duration,
    pub cycles_per_second: u16,
    pub use_german_keyboard_layout: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cycle_time: Duration::from_millis(1000 / 60),
            cycles_per_second: 60,
            use_german_keyboard_layout: true,
        }
    }
}
