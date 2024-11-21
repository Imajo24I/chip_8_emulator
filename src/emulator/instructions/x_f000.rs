use crate::emulator::emulator::Emulator;
use crate::emulator::instructions::{get_v_reg_value, unknown_instruction_err, validate_v_reg_index};
use crate::errors::error::{Cause, Error};
use crate::events::Event;

pub fn x_f000(emulator: &mut Emulator, opcode: u16) -> Result<(), Event> {
    match opcode & 0x00FF {
        0x0007 => {
            // FX07 - Set VX to value of delay timer
            let vx = ((opcode & 0x0F00) >> 8) as usize;
            validate_v_reg_index(vx, opcode, emulator)?;

            emulator.v_registers[vx] = emulator.delay_timer;
        }

        0x0015 => {
            // FX15 - Set delay timer to value of VX
            emulator.delay_timer =
                get_v_reg_value(((opcode & 0x0F00) >> 8) as usize, opcode, emulator)?;
        }

        0x0018 => {
            // FX18 - Set sound timer to value of VX
            emulator.sound_timer =
                get_v_reg_value(((opcode & 0x0F00) >> 8) as usize, opcode, emulator)?;
        }

        0x001E => {
            // FX1E - Add value of VX to I
            emulator.i_register +=
                get_v_reg_value(((opcode & 0x0F00) >> 8) as usize, opcode, emulator)? as usize;
        }

        0x000A => {
            // FX0A - Wait for keypress and store it in VX
            let vx = ((opcode & 0x0F00) >> 8) as usize;
            validate_v_reg_index(vx, opcode, emulator)?;

            // TODO: implement keypad and this

            emulator.pc -= 2;
        }

        0x0029 => {
            // FX29 - Set I to location of sprite for hex digit value of VX
            let x = get_v_reg_value(((opcode & 0x0F00) >> 8) as usize, opcode, emulator)?;
            emulator.i_register = x as usize * 5;
        }

        0x0033 => {
            // FX33 - Store the binary-coded decimal representation of VX at address I
            let x = get_v_reg_value(((opcode & 0x0F00) >> 8) as usize, opcode, emulator)?;

            if emulator.i_register + 2 > 0x0FFF {
                i_reg_out_of_bounds_err(2, opcode, emulator)?;
            }

            emulator.memory[emulator.i_register] = x / 100;
            emulator.memory[emulator.i_register + 1] = (x % 100) / 10;
            emulator.memory[emulator.i_register + 2] = x % 10;
        }

        0x0055 => {
            // FX55 - Store registers V0 to VX in memory starting at address I
            let x = get_v_reg_value(((opcode & 0x0F00) >> 8) as usize, opcode, emulator)?;

            if x > 16 {
                return Err(Event::ReportErrorAndExit(Error::new(
                    "Error executing program - Please ensure its a valid Chip 8 Program".to_string(),
                    Cause::new(
                        Some(format!("Invalid instruction parameters - No variable register with index {} exists - Instruction {:#06x} is located at memory location {}", x, opcode, emulator.pc - 2)),
                        None,
                    ),
                )));
            }

            if emulator.i_register + x as usize > 0x0FFF {
                i_reg_out_of_bounds_err(x as usize, opcode, emulator)?;
            }

            for vy in 0..x {
                emulator.memory[emulator.i_register + vy as usize] = emulator.v_registers[vy as usize];
            }
        }

        0x0065 => {
            // FX65 - Read registers V0 to VX from memory starting at address I
            let x = get_v_reg_value(((opcode & 0x0F00) >> 8) as usize, opcode, emulator)?;

            if x > 16 {
                return Err(Event::ReportErrorAndExit(Error::new(
                    "Error executing program - Please ensure its a valid Chip 8 Program".to_string(),
                    Cause::new(
                        Some(format!("Invalid instruction parameters - No variable register with index {} exists - Instruction {:#06x} is located at memory location {}", x, opcode, emulator.pc - 2)),
                        None,
                    ),
                )));
            }

            if emulator.i_register + x as usize > 0x0FFF {
                i_reg_out_of_bounds_err(x as usize, opcode, emulator)?;
            }

            for vy in 0..x {
                emulator.v_registers[vy as usize] = emulator.memory[emulator.i_register + vy as usize];
            }
        }

        _ => unknown_instruction_err(emulator, opcode)?,
    }

    Ok(())
}

fn i_reg_out_of_bounds_err(i_reg_shift: usize, opcode: u16, emulator: &mut Emulator) -> Result<(), Event> {
    Err(Event::ReportErrorAndExit(Error::new(
        "Error executing program - Please ensure its a valid Chip 8 Program".to_string(),
        Cause::new(
            Some(format!("I register with value of {} is out of bounds - Instruction {:#06x} is located at memory location {}", emulator.i_register + i_reg_shift, opcode, emulator.pc - 2)),
            None,
        ),
    )))
}
