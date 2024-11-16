use crate::errors::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::emulator::opcodes;
use crate::events::Event;

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

const MEMORY_SIZE: usize = 4096;
const INSTRUCTIONS_START: usize = 0x200;
const FONT_START: usize = 0x50;
const FONT_END: usize = 0x0A0;

pub struct Emulator {
    pub display: [[bool; 64]; 32],

    // Memory
    // 4096 bytes of memory
    memory: [u8; MEMORY_SIZE],

    // Program Counter
    // Used to store location of the next instruction
    pub(crate) pc: usize,

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
                        Error::new_with_cause(
                            format!("Error reading file at {} - Please ensure it is a valid file.", filepath.display()),
                            Box::new(error),
                        )
                    );
                }

                if data.len() > MEMORY_SIZE - INSTRUCTIONS_START {
                    return Err(Error::new(format!(
                        "Error reading file at {} - File exceeds maximum data size of {} bytes.",
                        filepath.display(), MEMORY_SIZE - INSTRUCTIONS_START
                    )));
                }

                // Add instructions and font to memory
                memory[INSTRUCTIONS_START..INSTRUCTIONS_START + data.len()].copy_from_slice(&data);
                memory[FONT_START..FONT_END].copy_from_slice(&FONT_BYTES);
            }

            Err(error) => {
                return Err(
                    Error::new_with_cause(
                        format!("Error opening file at {} - Please ensure the path points to a valid file", filepath.display()),
                        Box::new(error),
                    )
                )
            }
        }

        Ok(memory)
    }

    pub fn run_cycle(&mut self) -> Option<Event> {
        // Exit if no more instructions left
        if self.pc >= MEMORY_SIZE {
            return Some(Event::Exit);
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

        opcodes::execute_opcode(self, opcode)?;

        None
    }

    fn make_sound(&mut self) {
        // TODO: Implement this
    }
}