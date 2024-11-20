use crate::emulator::emulator::Emulator;
use crate::errors::error::{Cause, Error};
use crate::events::Event;

pub fn execute_instruction(emulator: &mut Emulator, opcode: u16) -> Result<(), Event> {
    match opcode & 0xF000 {
        0x0000 => {
            match opcode & 0x00FF {
                0x00E0 => {
                    // 00E0 - Clear display
                    emulator.display.fill_with(|| [false; 64]);
                }

                _ => {
                    return unknown_instruction_err(emulator, opcode);
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
            validate_v_reg_index(vx, opcode, emulator)?;
            emulator.v_registers[vx] = (opcode & 0x00FF) as u8;
        }

        0x7000 => {
            // 7XNN - Add NN to VX
            let vx = ((opcode & 0x0F00) >> 8) as usize;
            validate_v_reg_index(vx, opcode, emulator)?;

            let nn = opcode & 0x00FF;
            if nn > 255 {
                emulator.v_registers[vx] = 255;
            } else {
                emulator.v_registers[vx] += nn as u8;
            }
        }

        0xA000 => {
            // ANNN - Set index register to NNN
            emulator.i_register = opcode & 0x0FFF;
        }

        0xD000 => {
            return x_dxyn(emulator, opcode);
        }

        _ => {
            return unknown_instruction_err(emulator, opcode);
        }
    }

    Ok(())
}

fn x_dxyn(emulator: &mut Emulator, opcode: u16) -> Result<(), Event> {
    // DXYN - Draw sprite at coordinate VX, VY with N bytes of sprite data

    let vx = ((opcode & 0x0F00) >> 8) as usize;
    let vy = ((opcode & 0x00F0) >> 4) as usize;

    for v in [vx, vy].iter() {
        validate_v_reg_index(*v, opcode, emulator)?;
    }

    let x = (emulator.v_registers[vx] & 63) as usize;
    let y = (emulator.v_registers[vy] & 31) as usize;

    emulator.v_registers[0xF] = 0;


    let n = (opcode & 0x000F) as usize;

    for row in 0..n {
        let sprite_data = emulator.memory[emulator.i_register as usize + row];
        let mut bit_x = x;
        for bit in (0..8).rev() {
            let current_bit = (sprite_data >> bit) & 1;
            emulator.display[y + row][bit_x] = current_bit == 1;

            if current_bit == 1 {
                emulator.v_registers[0xF] = 1;
            }

            bit_x += 1;
            if bit_x == 64 {
                break;
            }
        }

        if y + row == 32 {
            break;
        }
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

fn validate_v_reg_index(vx: usize, opcode: u16, emulator: &mut Emulator) -> Result<(), Event> {
    if vx > 15 {
        Err(Event::ReportErrorAndExit(Error::new(
            "Error executing program - Please ensure its a valid Chip 8 Program".to_string(),
            Cause::new(
                Some(format!("Invalid instruction parameters - No variable register with index {} exists - Instruction {:#06x} is located at memory location {}", vx,opcode , emulator.pc - 2)),
                None,
            ),
        )))
    } else {
        Ok(())
    }
}
