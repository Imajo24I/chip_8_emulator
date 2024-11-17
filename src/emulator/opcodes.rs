use crate::emulator::emulator::Emulator;
use crate::errors::error::{Cause, Error};
use crate::events::Event;

pub fn execute_opcode(emulator: &mut Emulator, opcode: u16) -> Option<Event> {
    match opcode & 0xF000 {
        0x0000 => {
            match opcode & 0x00FF {
                0x00E0 => {
                    // 00E0 - Clear display
                    emulator.display.fill_with(|| [false; 64]);
                }

                _ => {
                    return report_exit_unknown_instruction(emulator, opcode);
                }
            }
        }

        0x1000 => {
            // 1NNN - Jump to NNN
            emulator.pc = (opcode & 0x0FFF) as usize;
        }

        0x6000 => {
            // 6XNN - Set VX to NN
            let vx = ((opcode & 0x0F00) >> 8) as usize;

            if vx > 15 {
                return Some(Event::ReportErrorAndExit(Error::new(
                    "Error executing program - Please ensure its a valid Chip 8 Program".to_string(),
                    Cause::new(
                        Some(format!("Invalid instruction parameters - No variable register with index {} exists - Instruction {:#06x} is located at memory location {}", vx, opcode, emulator.pc - 2)),
                        None,
                    ),
                )));
            }

            emulator.v_registers[vx] = (opcode & 0x00FF) as u8;
        }

        _ => {
            return report_exit_unknown_instruction(emulator, opcode);
        }
    }

    None
}

fn report_exit_unknown_instruction(emulator: &mut Emulator, opcode: u16) -> Option<Event> {
    Some(Event::ReportErrorAndExit(
        Error::new(
            "Error executing program - Please ensure its a valid Chip 8 Program".to_string(),
            Cause::new(
                Some(format!("Unknown instruction: {:#06x} - Instruction is located at memory location {}", opcode, emulator.pc - 2)),
                None,
            ),
        ),
    ))
}
