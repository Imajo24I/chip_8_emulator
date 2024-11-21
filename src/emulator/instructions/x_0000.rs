use crate::emulator::emulator::Emulator;
use crate::emulator::instructions::unknown_instruction_err;
use crate::errors::error::{Cause, Error};
use crate::events::Event;

pub fn x_0000(emulator: &mut Emulator, opcode: u16) -> Result<(), Event> {
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
                    return Err(Event::ReportErrorAndExit(Error::new(
                        "Error executing program - Please ensure its a valid Chip 8 Program".to_string(),
                        Cause::new(
                            Some(format!("No subroutine to return from - Instruction {:#06x} is located at memory location {}", opcode, emulator.pc - 2)),
                            None,
                        ),
                    )));
                }
            }
        }

        _ => {
            return unknown_instruction_err(emulator, opcode);
        }
    }

    Ok(())
}