use std::path::PathBuf;

pub struct Chip8Emulator {
    pub display: [[bool; 64]; 32],

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

impl Chip8Emulator {
    pub fn new(filepath: PathBuf) -> Self {
        Self {
            display: [[false; 64]; 32],
            // Start at 0x200, since 0x000 - 0x1FF are reserved for interpreter
            pc: 0x200,
            i_register: 0,
            stack: Vec::new(),
            v_registers: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
        }
    }
}