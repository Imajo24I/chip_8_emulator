mod x_8000;
mod x_0000;
mod x_f000;
mod x_dxyn;
mod x_e000;

use eframe::egui::InputState;
use crate::emulator::emulator::Emulator;
use crate::emulator::instructions::x_f000::x_f000;
use crate::errors::error::{Cause, Error};
use crate::events::Event;

pub fn execute_instruction(emulator: &mut Emulator, opcode: u16, input_state: &InputState) -> Result<(), Event> {
    match opcode & 0xF000 {
        0x0000 => x_0000::x_0000(emulator, opcode)?,

        0x1000 => {
            // 1NNN - Jump to NNN
            emulator.pc = (opcode & 0x0FFF) as usize;
        }

        0x2000 => {
            // 2NNN - Call subroutine at NNN
            emulator.stack.push(emulator.pc);
            emulator.pc = (opcode & 0x0FFF) as usize;
        }

        0x3000 => {
            // 3XNN - Skip next instruction if VX == NN
            if get_v_reg_value(((opcode & 0x0F00) >> 8) as usize, opcode, emulator)?
                == (opcode & 0x00FF) as u8 {
                emulator.pc += 2;
            }
        }

        0x4000 => {
            // 4XNN - Skip next instruction if VX != NN
            if get_v_reg_value(((opcode & 0x0F00) >> 8) as usize, opcode, emulator)?
                != (opcode & 0x00FF) as u8 {
                emulator.pc += 2;
            }
        }

        0x5000 => {
            // 5XY0 - Skip next instruction of VX == VY
            if get_v_reg_value(((opcode & 0x0F00) >> 8) as usize, opcode, emulator)?
                == get_v_reg_value(((opcode & 0x00F0) >> 4) as usize, opcode, emulator)? {
                emulator.pc += 2;
            }
        }

        0x6000 => {
            // 6XNN - Set VX to NN
            let vx = ((opcode & 0x0F00) >> 8) as usize;
            validate_v_reg_index(vx, opcode, emulator)?;
            emulator.v_registers[vx] = (opcode & 0x00FF) as u8;
        }

        0x7000 => {
            // 7XNN - Add NN to VX
            let vx = ((opcode & 0x0F00) >> 8) as usize;
            validate_v_reg_index(vx, opcode, emulator)?;

            let nn = opcode & 0x00FF;
            let sum = emulator.v_registers[vx] as u16 + nn;

            if sum > 255 {
                emulator.v_registers[vx] = (sum - 256) as u8;
            } else {
                emulator.v_registers[vx] = sum as u8;
            }
        }

        0x8000 => x_8000::x_8000(emulator, opcode)?,

        0x9000 => {
            // 9XY0 - Skip next instruction of VX != VY
            if get_v_reg_value(((opcode & 0x0F00) >> 8) as usize, opcode, emulator)?
                != get_v_reg_value(((opcode & 0x00F0) >> 4) as usize, opcode, emulator)? {
                emulator.pc += 2;
            }
        }

        0xA000 => {
            // ANNN - Set I to NNN
            emulator.i_register = (opcode & 0x0FFF) as usize;
        }

        0xD000 => x_dxyn::x_dxyn(emulator, opcode)?,

        0xE000 => x_e000::x_e000(emulator, opcode, input_state)?,

        0xF000 => x_f000(emulator, opcode, input_state)?,

        _ => unknown_instruction_err(emulator, opcode)?,
    }

    Ok(())
}

fn unknown_instruction_err(emulator: &mut Emulator, opcode: u16) -> Result<(), Event> {
    Err(Event::ReportErrorAndExit(
        Error::new(
            "Error executing program - Please ensure its a valid Chip 8 Program".to_string(),
            Cause::new(
                Some(format!("Unknown instruction: {:#06x} - Instruction is located at memory location {}", opcode, emulator.pc - 2)),
                None,
            ),
        ),
    ))
}

fn get_v_reg_value(vx: usize, opcode: u16, emulator: &mut Emulator) -> Result<u8, Event> {
    validate_v_reg_index(vx, opcode, emulator)?;
    Ok(emulator.v_registers[vx])
}

fn validate_v_reg_index(vx: usize, opcode: u16, emulator: &mut Emulator) -> Result<(), Event> {
    if vx > 15 {
        Err(Event::ReportErrorAndExit(Error::new(
            "Error executing program - Please ensure its a valid Chip 8 Program".to_string(),
            Cause::new(
                Some(format!("Invalid instruction parameters - No variable register with index {} exists - Instruction {:#06x} is located at memory location {}", vx, opcode, emulator.pc - 2)),
                None,
            ),
        )))
    } else {
        Ok(())
    }
}
