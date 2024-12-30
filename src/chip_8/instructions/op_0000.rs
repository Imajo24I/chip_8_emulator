use crate::chip_8::emulator::Emulator;
use crate::chip_8::instructions::unknown_instruction_err;
use anyhow::{anyhow, Result};
use crate::chip_8::display::Resolution;

pub fn op_0000(emulator: &mut Emulator, opcode: u16) -> Result<()> {
    match opcode & 0x00FF {
        0x00E0 => {
            // 00E0 - Clear display
            emulator.display.clear();
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

        0x00FE => {
            // SuperChip Instruction
            // 00FE - Set resolution to 64x32
            emulator.display.set_resolution(Resolution::Lores);
        }

        0x00FF => {
            // SuperChip Instruction
            // 00FF - Set resolution to 128x64
            emulator.display.set_resolution(Resolution::Hires);
        }

        _ => {
            return unknown_instruction_err(emulator, opcode);
        }
    }

    Ok(())
}
