use crate::emulator::emulator::Emulator;
use crate::errors::error::{Cause, Error};
use crate::events::Event;

pub fn execute_instruction(emulator: &mut Emulator, opcode: u16) -> Result<(), Event> {
    match opcode & 0xF000 {
        0x0000 => x_0000(emulator, opcode)?,

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
            if emulator.v_registers[vx] as u16 + nn > 255 {
                emulator.v_registers[vx] = 255;
            } else {
                emulator.v_registers[vx] += nn as u8;
            }
        }

        0x8000 => x_8000(emulator, opcode)?,

        0x9000 => {
            // 9XY0 - Skip next instruction of VX != VY
            if get_v_reg_value(((opcode & 0x0F00) >> 8) as usize, opcode, emulator)?
                != get_v_reg_value(((opcode & 0x00F0) >> 4) as usize, opcode, emulator)? {
                emulator.pc += 2;
            }
        }

        0xA000 => {
            // ANNN - Set index register to NNN
            emulator.i_register = opcode & 0x0FFF;
        }

        0xD000 => x_dxyn(emulator, opcode)?,

        _ => unknown_instruction_err(emulator, opcode)?,
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

fn x_0000(emulator: &mut Emulator, opcode: u16) -> Result<(), Event> {
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

fn x_8000(emulator: &mut Emulator, opcode: u16) -> Result<(), Event> {
    match opcode & 0x000F {
        0x0000 => {
            // 8XY0 - Set VX to value of VY
            let vx = ((opcode & 0x0F00) >> 8) as usize;
            validate_v_reg_index(vx, opcode, emulator)?;

            emulator.v_registers[vx] =
                get_v_reg_value(((opcode & 0x00F0) >> 4) as usize, opcode, emulator)?;
        }

        0x0001 => {
            // 8XY1 - Set VX to the binary OR value of VX and VY
            let vx = ((opcode & 0x0F00) >> 8) as usize;
            validate_v_reg_index(vx, opcode, emulator)?;

            emulator.v_registers[vx] =
                get_v_reg_value(vx, opcode, emulator)? |
                    get_v_reg_value(((opcode & 0x00F0) >> 4) as usize, opcode, emulator)?;
        }

        0x0002 => {
            // 8XY2 - Set VX to the binary AND value of VX and VY
            let vx = ((opcode & 0x0F00) >> 8) as usize;
            validate_v_reg_index(vx, opcode, emulator)?;

            emulator.v_registers[vx] =
                get_v_reg_value(vx, opcode, emulator)? &
                    get_v_reg_value(((opcode & 0x00F0) >> 4) as usize, opcode, emulator)?;
        }

        0x0003 => {
            // 8XY3 - Set VX to the binary XOR value of VX and VY
            let vx = ((opcode & 0x0F00) >> 8) as usize;
            validate_v_reg_index(vx, opcode, emulator)?;

            emulator.v_registers[vx] =
                get_v_reg_value(vx, opcode, emulator)? ^
                    get_v_reg_value(((opcode & 0x00F0) >> 4) as usize, opcode, emulator)?;
        }

        0x0004 => {
            // 8XY4 - Set VX to the sum of VX and VY
            let vx = ((opcode & 0x0F00) >> 8) as usize;
            validate_v_reg_index(vx, opcode, emulator)?;

            let sum = get_v_reg_value(vx, opcode, emulator)? as u16 +
                get_v_reg_value(((opcode & 0x00F0) >> 4) as usize, opcode, emulator)? as u16;

            if sum > 255 {
                emulator.v_registers[0xF] = 1;
                emulator.v_registers[vx] = 255;
            } else {
                emulator.v_registers[0xF] = 0;
                emulator.v_registers[vx] = sum as u8;
            }
        }

        0x0005 => {
            // 8XY5 - Set VX to the difference of VX and VY
            let vx = ((opcode & 0x0F00) >> 8) as usize;
            validate_v_reg_index(vx, opcode, emulator)?;

            let diff = get_v_reg_value(vx, opcode, emulator)? as i16 -
                get_v_reg_value(((opcode & 0x00F0) >> 4) as usize, opcode, emulator)? as i16;

            if diff < 0 {
                emulator.v_registers[0xF] = 0;
                emulator.v_registers[vx] = 0;
            } else {
                emulator.v_registers[0xF] = 1;
                emulator.v_registers[vx] = diff as u8;
            }
        }

        0x0006 => {
            // 8XY6 - Set VX to VY shifted by 1 to the right. Set VF to the shifted out bit.
            let vx = ((opcode & 0x0F00) >> 8) as usize;
            validate_v_reg_index(vx, opcode, emulator)?;

            let y = get_v_reg_value(((opcode & 0x00F0) >> 4) as usize, opcode, emulator)?;

            let shifted_out_bit = y & 1;
            emulator.v_registers[0xF] = shifted_out_bit;

            emulator.v_registers[vx] = y >> 1;
        }

        0x0007 => {
            // 8XY7 - Set VX to the difference of VY and VX
            let vx = ((opcode & 0x0F00) >> 8) as usize;
            validate_v_reg_index(vx, opcode, emulator)?;

            let diff = get_v_reg_value(((opcode & 0x00F0) >> 4) as usize, opcode, emulator)? as i16 -
                get_v_reg_value(vx, opcode, emulator)? as i16;

            if diff < 0 {
                emulator.v_registers[0xF] = 0;
                emulator.v_registers[vx] = 0;
            } else {
                emulator.v_registers[0xF] = 1;
                emulator.v_registers[vx] = diff as u8;
            }
        }

        0x000E => {
            // 8XY6 - Set VX to VY shifted by 1 to the left. Set VF to the shifted out bit.
            let vx = ((opcode & 0x0F00) >> 8) as usize;
            validate_v_reg_index(vx, opcode, emulator)?;

            let y = get_v_reg_value(((opcode & 0x00F0) >> 4) as usize, opcode, emulator)?;

            let shifted_out_bit = y >> 7;
            emulator.v_registers[0xF] = shifted_out_bit;

            emulator.v_registers[vx] = y << 1;
        }

        _ => unknown_instruction_err(emulator, opcode)?
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
