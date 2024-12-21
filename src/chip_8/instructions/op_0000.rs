use crate::chip_8::emulator::Emulator;
use crate::chip_8::instructions::unknown_instruction_err;
use anyhow::{anyhow, Result};

pub fn op_0000(emulator: &mut Emulator, opcode: u16) -> Result<()> {
    match opcode & 0x00FF {
        0x00E0 => {
            // 00E0 - Clear display
            emulator.display.fill_with(|| [false; 64]);
        }

        0x00EE => {
            // 00EE - Return from subroutine
            match emulator.stack.pop() {
                Some(pc) => emulator.pc = pc,
                None => {
                    return Err(anyhow!("No subroutine to return from\nInstruction {:#06x} is located at memory location {}", opcode, emulator.pc - 2));
                }
            }
        }

        _ => {
            return unknown_instruction_err(emulator, opcode);
        }
    }

    Ok(())
}
