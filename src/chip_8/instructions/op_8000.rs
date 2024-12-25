use crate::chip_8::emulator::Emulator;
use crate::chip_8::instructions::unknown_instruction_err;
use anyhow::Result;

pub fn op_8000(emulator: &mut Emulator, opcode: u16) -> Result<()> {
    match opcode & 0x000F {
        0x0000 => {
            // 8XY0 - Set VX to value of VY
            let vx = ((opcode & 0x0F00) >> 8) as usize;
            emulator.v_registers[vx] =
                emulator.v_registers[((opcode & 0x00F0) >> 4) as usize];
        }

        0x0001 => {
            // 8XY1 - Set VX to the binary OR value of VX and VY
            let vx = ((opcode & 0x0F00) >> 8) as usize;

            if emulator.config.quirks.vf_reset {
                emulator.v_registers[0xF] = 0;
            }

            emulator.v_registers[vx] = emulator.v_registers[vx]
                | emulator.v_registers[((opcode & 0x00F0) >> 4) as usize]
        }

        0x0002 => {
            // 8XY2 - Set VX to the binary AND value of VX and VY
            let vx = ((opcode & 0x0F00) >> 8) as usize;

            if emulator.config.quirks.vf_reset {
                emulator.v_registers[0xF] = 0;
            }

            emulator.v_registers[0xF] = emulator.v_registers[vx]
                & emulator.v_registers[((opcode & 0x00F0) >> 4) as usize];
        }

        0x0003 => {
            // 8XY3 - Set VX to the binary XOR value of VX and VY
            let vx = ((opcode & 0x0F00) >> 8) as usize;

            if emulator.config.quirks.vf_reset {
                emulator.v_registers[0xF] = 0;
            }

            emulator.v_registers[vx] = emulator.v_registers[vx]
                & emulator.v_registers[((opcode & 0x00F0) >> 4) as usize];
        }

        0x0004 => {
            // 8XY4 - Set VX to the sum of VX and VY
            let vx = ((opcode & 0x0F00) >> 8) as usize;

            let sum = emulator.v_registers[vx] as u16
                + emulator.v_registers[((opcode & 0x00F0) >> 4) as usize] as u16;

            if sum > 255 {
                emulator.v_registers[vx] = (sum - 256) as u8;
                emulator.v_registers[0xF] = 1;
            } else {
                emulator.v_registers[vx] = sum as u8;
                emulator.v_registers[0xF] = 0;
            }
        }

        0x0005 => {
            // 8XY5 - Set VX to the difference of VX and VY
            let vx = ((opcode & 0x0F00) >> 8) as usize;

            let diff = emulator.v_registers[vx] as i16
                - emulator.v_registers[((opcode & 0x00F0) >> 4) as usize] as i16;

            if diff < 0 {
                emulator.v_registers[vx] = (diff + 256) as u8;
                emulator.v_registers[0xF] = 0;
            } else {
                emulator.v_registers[vx] = diff as u8;
                emulator.v_registers[0xF] = 1;
            }
        }

        0x0006 => {
            // 8XY6 - Set VX to VY shifted by 1 to the right. Set VF to the shifted out bit.
            // If the shift_vx_directly quirk is active, shift VX directly, without setting VX to VY
            let vx = ((opcode & 0x0F00) >> 8) as usize;
            let value = shift_vx_quirk(vx, opcode, emulator)?;
            let shifted_out_bit = value & 1;

            emulator.v_registers[vx] = value >> 1;
            emulator.v_registers[0xF] = shifted_out_bit;
        }

        0x0007 => {
            // 8XY7 - Set VX to the difference of VY and VX
            let vx = ((opcode & 0x0F00) >> 8) as usize;

            let diff = emulator.v_registers[((opcode & 0x00F0) >> 4) as usize] as i16
                - emulator.v_registers[vx] as i16;

            if diff < 0 {
                emulator.v_registers[vx] = (diff - 256) as u8;
                emulator.v_registers[0xF] = 0;
            } else {
                emulator.v_registers[vx] = diff as u8;
                emulator.v_registers[0xF] = 1;
            }
        }

        0x000E => {
            // 8XYE - Set VX to VY shifted by 1 to the left. Set VF to the shifted out bit.
            // If the shift_vx_directly quirk is active, shift VX directly, without setting VX to VY
            let vx = ((opcode & 0x0F00) >> 8) as usize;
            let value = shift_vx_quirk(vx, opcode, emulator)?;
            let shifted_out_bit = value >> 7;

            emulator.v_registers[vx] = value << 1;
            emulator.v_registers[0xF] = shifted_out_bit;
        }

        _ => unknown_instruction_err(emulator, opcode)?,
    }

    Ok(())
}

fn shift_vx_quirk(vx: usize, opcode: u16, emulator: &mut Emulator) -> Result<u8> {
    if !emulator.config.quirks.shift_vx_directly {
        Ok(emulator.v_registers[((opcode & 0x00F0) >> 4) as usize])
    } else {
        Ok(emulator.v_registers[vx])
    }
}
