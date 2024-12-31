use crate::chip_8::config::Config;
use crate::chip_8::instructions;
use crate::events::Event;
use crate::chip_8::beep::{Beeper, BeeperSettings};
use crate::chip_8::display::Display;
use crate::chip_8::keypad::Keypad;
use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::chip_8::font::{LARGE_FONT, SMALL_FONT};

pub const MEMORY_SIZE: usize = 4096;
// Instructions start at 0x200, since 0x000 - 0x1FF are reserved for interpreter
const INSTRUCTIONS_START: usize = 0x200;

#[derive(Clone)]
pub struct Emulator {
    pub config: Config,

    pub beeper: Beeper,

    pub display: Display,

    pub keypad: Keypad,

    // Memory
    // 4096 bytes of memory
    pub memory: [u8; MEMORY_SIZE],

    // Program Counter
    // Used to store location of the next instruction
    pub pc: usize,

    // Index Register
    // Used to point at locations in memory
    pub i_reg: usize,

    // Stack
    // Used to call and return from subroutines (functions)
    pub stack: Vec<usize>,

    // General-purpose variable registers
    pub v_regs: [u8; 16],

    // Flag Registers
    // Persistent across program runs
    // TODO: Make them actually persistent
    pub f_regs: [u8; 16],

    // Delay Timer
    // Decrements 60 times per second
    pub delay_timer: u8,

    // Sound Timer
    // Functions like the Delay Timer, however also gives of a beep sounds when not 0
    pub sound_timer: u8,
}

impl Default for Emulator {
    fn default() -> Self {
        let config = Config::default();

        Self {
            config,
            beeper: Beeper::new(BeeperSettings::default()),
            display: Display::default(),
            keypad: Keypad::new(config.use_german_keyboard_layout),
            pc: INSTRUCTIONS_START,
            i_reg: 0,
            stack: Vec::new(),
            v_regs: [0; 16],
            f_regs: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            memory: [0; MEMORY_SIZE],
        }
    }
}

impl Emulator {
    pub fn initialize_memory(&mut self, filepath: &Path) -> Result<()> {
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

                // Insert program into memory
                self.memory[INSTRUCTIONS_START..INSTRUCTIONS_START + data.len()]
                    .copy_from_slice(&data);

                // Insert fonts into memory
                let small_font_len = SMALL_FONT.len();
                let large_font_len = LARGE_FONT.len();

                self.memory[0..small_font_len].copy_from_slice(&SMALL_FONT);
                self.memory[small_font_len..small_font_len + large_font_len].copy_from_slice(&LARGE_FONT);
            }

            Err(error) => {
                return Err(anyhow!(error).context(format!(
                    "Error opening file at {}\nPlease ensure the path points to a valid file",
                    filepath.display()
                )))
            }
        }

        Ok(())
    }

    pub fn run_cycle(&mut self) -> Result<(), Event> {
        // Exit if no more instructions left
        if self.pc >= MEMORY_SIZE {
            self.beeper.stop();
            return Err(Event::Exit);
        }

        // Fetch opcode
        let opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
        self.pc += 2;

        // Execute instruction
        if let Err(error) = instructions::execute_instruction(self, opcode) {
            Err(Event::ReportError(error))
        } else {
            Ok(())
        }
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;

            if self.sound_timer == 0 {
                self.beeper.pause();
            }
        }
    }
}
