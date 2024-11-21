use crate::emulator::emulator::Emulator;
use crate::emulator::instructions::{get_v_reg_value, unknown_instruction_err, validate_v_reg_index};
use crate::events::Event;

pub fn x_8000(emulator: &mut Emulator, opcode: u16) -> Result<(), Event> {
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
                emulator.v_registers[vx] = (sum - 256) as u8;
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
                emulator.v_registers[vx] = (diff + 256) as u8;
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
                emulator.v_registers[vx] = (diff - 256) as u8;
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
