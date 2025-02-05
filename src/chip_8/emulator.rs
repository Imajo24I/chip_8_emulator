use crate::chip_8::config::Config;
use crate::chip_8::display::Display;
use crate::chip_8::instructions;
use crate::chip_8::keypad::Keypad;
use crate::chip_8::memory::Memory;
use crate::chip_8::sound::Beeper;
use crate::emulator_app::Event;
use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

// Instructions start at 0x200, since 0x000 - 0x1FF are reserved for interpreter
pub const INSTRUCTIONS_START: usize = 0x200;

#[derive(Clone)]
pub struct Emulator {
    pub config: Config,

    pub beeper: Beeper,

    pub display: Display,

    pub keypad: Keypad,

    // Memory
    pub memory: Memory,
    pub rom_loaded: bool,

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
        Self::new(Config::default())
    }
}

impl Emulator {
    pub fn new(config: Config) -> Self {
        let mut memory = Memory::default();
        memory.load_fonts();

        Self {
            memory,
            config,
            beeper: Beeper::default(),
            display: Display::default(),
            keypad: Keypad::default(),
            pc: INSTRUCTIONS_START,
            i_reg: 0,
            stack: Vec::new(),
            v_regs: [0; 16],
            f_regs: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            rom_loaded: false
        }
    }

    pub fn select_rom(&mut self, filepath: PathBuf) {
        self.config.filepath = Some(filepath);
    }

    pub fn get_rom(&self) -> &Option<PathBuf> {
        &self.config.filepath
    }

    pub fn load_rom(&mut self) -> Result<()> {
        if let Some(filepath) = &self.config.filepath {
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

                    if data.len() > self.memory.size - INSTRUCTIONS_START {
                        return Err(anyhow!(
                            "File with size of {} bytes exceeds maximum data size of {} bytes.",
                            data.len(),
                            self.memory.size - INSTRUCTIONS_START
                        ));
                    }

                    self.memory.load_rom(&data);
                    self.rom_loaded = true;
                }

                Err(error) => {
                    return Err(anyhow!(error).context(format!(
                        "Error opening file at {}\nPlease ensure the path points to a valid file",
                        filepath.display()
                    )))
                }
            }
        } else {
            return Err(anyhow!("Trying to load ROM without any selected"));
        }

        Ok(())
    }

    pub fn reset(&mut self) {
        self.rom_loaded = false;

        self.beeper.stop();
        let keypad = self.keypad.clone();

        *self = Emulator::new(self.config.clone());

        // Set keypad to previous, due to keybindings
        self.keypad = keypad;
    }

    pub fn execute_instruction(&mut self) -> Result<(), Event> {
        // Exit if no more instructions left
        if self.pc >= self.memory.size {
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

    /// Skip the next instruction
    /// If the next instruction is F000, this will skip 4 bytes instead of 2
    pub fn skip_instruction(&mut self) {
        let opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
        if opcode == 0xF000 {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }
}
