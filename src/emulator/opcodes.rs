use crate::emulator::emulator::Emulator;
use crate::errors::error::Error;
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
            // 1NNN - Jump
            emulator.pc = (opcode & 0x0FFF) as usize;
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
            format!("Unknown Instruction: {:#06x} - Located at memory location {}", opcode, emulator.pc - 2)
        )
    ))
}
