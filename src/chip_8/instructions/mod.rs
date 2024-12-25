mod op_0000;
mod op_8000;
mod op_dxyn;
mod op_e000;
mod op_f000;

use crate::chip_8::emulator::Emulator;
use crate::chip_8::instructions::op_f000::op_f000;
use anyhow::{anyhow, Result};

pub fn execute_instruction(emulator: &mut Emulator, opcode: u16) -> Result<()> {
    match opcode & 0xF000 {
        0x0000 => op_0000::op_0000(emulator, opcode)?,

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
                == (opcode & 0x00FF) as u8
            {
                emulator.pc += 2;
            }
        }

        0x4000 => {
            // 4XNN - Skip next instruction if VX != NN
            if get_v_reg_value(((opcode & 0x0F00) >> 8) as usize, opcode, emulator)?
                != (opcode & 0x00FF) as u8
            {
                emulator.pc += 2;
            }
        }

        0x5000 => {
            // 5XY0 - Skip next instruction of VX == VY
            if get_v_reg_value(((opcode & 0x0F00) >> 8) as usize, opcode, emulator)?
                == get_v_reg_value(((opcode & 0x00F0) >> 4) as usize, opcode, emulator)?
            {
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

        0x8000 => op_8000::op_8000(emulator, opcode)?,

        0x9000 => {
            // 9XY0 - Skip next instruction of VX != VY
            if get_v_reg_value(((opcode & 0x0F00) >> 8) as usize, opcode, emulator)?
                != get_v_reg_value(((opcode & 0x00F0) >> 4) as usize, opcode, emulator)?
            {
                emulator.pc += 2;
            }
        }

        0xA000 => {
            // ANNN - Set I to NNN
            emulator.i_register = (opcode & 0x0FFF) as usize;
        }

        0xB000 => {
            let reg_value = if !emulator.config.quirks.vx_offset_jump {
                // BNNN - Jump to NNN + V0
                emulator.v_registers[0]
            } else {
                // BXNN - Jump to NNN + VX
                get_v_reg_value(((opcode & 0x0F00) >> 8) as usize, opcode, emulator)?
            } ;

            emulator.pc = ((opcode & 0x0FFF) + reg_value as u16) as usize;
        }

        0xC000 => {
            // CXNN - Binary AND a random number with NN and set VX to the number
            let vx = ((opcode & 0x0F00) >> 8) as usize;
            validate_v_reg_index(vx, opcode, emulator)?;

            emulator.v_registers[vx] = rand::random::<u8>() & (opcode & 0x00FF) as u8;
        }

        0xD000 => op_dxyn::op_dxyn(emulator, opcode)?,

        0xE000 => op_e000::op_e000(emulator, opcode)?,

        0xF000 => op_f000(emulator, opcode)?,

        _ => unknown_instruction_err(emulator, opcode)?,
    }

    Ok(())
}

fn unknown_instruction_err(emulator: &mut Emulator, opcode: u16) -> Result<()> {
    Err(anyhow!(
        "Unknown instruction: {:#06x}\nInstruction is located at memory location {}",
        opcode,
        emulator.pc - 2
    ))
}

fn get_v_reg_value(vx: usize, opcode: u16, emulator: &mut Emulator) -> Result<u8> {
    validate_v_reg_index(vx, opcode, emulator)?;
    Ok(emulator.v_registers[vx])
}

// TODO: is this even needed, since 4 bits are max 16?
fn validate_v_reg_index(vx: usize, opcode: u16, emulator: &mut Emulator) -> Result<()> {
    if vx > 15 {
        Err(
            anyhow!("Invalid instruction parameters - No variable register with index {} exists - Instruction {:#06x} is located at memory location {}", vx, opcode, emulator.pc - 2)
        )
    } else {
        Ok(())
    }
}
