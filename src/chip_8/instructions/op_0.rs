use crate::chip_8::display::Resolution;
use crate::chip_8::emulator::Emulator;
use crate::chip_8::instructions::unknown_instruction_err;
use anyhow::{anyhow, Result};

/// Execute instructions which start with 0
pub fn op_0(emulator: &mut Emulator, opcode: u16) -> Result<()> {
    match opcode & 0x00F0 {
        0x00C0 => {
            // SuperChip Instruction
            // 00CN - Scroll display down by N pixels
            let amount = (opcode & 0x000F) as usize;
            emulator.display.for_active_plane(|plane| plane.scroll_down(amount));
        }

        0x00D0 => {
            // XO-Chip Instruction
            // 00DN - Scroll display up by N pixels
            let amount = (opcode & 0x000F) as usize;
            emulator.display.for_active_plane(|plane| plane.scroll_up(amount));
        }

        0x00E0 => {
            match opcode & 0x000F {
                0x0000 => {
                    // 00E0 - Clear display
                    emulator.display.for_active_plane(|plane| plane.clear());
                }

                0x000E => {
                    // 00EE - Return from subroutine
                    match emulator.stack.pop() {
                        Some(pc) => emulator.pc = pc,
                        None => {
                            return Err(anyhow!("No subroutine to return from\nInstruction {:#06x} is located at memory location {}", opcode, emulator.pc - 2));
                        }
                    }
                }

                _ => unknown_instruction_err(emulator, opcode)?,
            }
        }

        0x00F0 => {
            match opcode & 0x000F {
                0x000B => {
                    // SuperChip Instruction
                    // 00FB - Scroll display right by 4 pixels
                    emulator.display.for_active_plane(|plane| plane.scroll_right());
                }

                0x0000C => {
                    // SuperChip Instruction
                    // 00FC - Scroll display left by 4 pixels
                    emulator.display.for_active_plane(|plane| plane.scroll_left());
                }

                0x000D => {
                    // SuperChip Instruction
                    // 00FD - Exit the program

                    // Emulator will automatically quit when reaching end of memory
                    emulator.pc = emulator.config.memory_size;
                }

                0x000E => {
                    // SuperChip Instruction
                    // 00FE - Set resolution to 64x32
                    emulator.display.set_resolution(Resolution::Low);
                }

                0x000F => {
                    // SuperChip Instruction
                    // 00FF - Set resolution to 128x64
                    emulator.display.set_resolution(Resolution::High);
                }

                _ => unknown_instruction_err(emulator, opcode)?,
            }
        }

        _ => unknown_instruction_err(emulator, opcode)?,
    }

    Ok(())
}
