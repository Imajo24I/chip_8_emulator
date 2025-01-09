mod op_0;
mod op_5;
mod op_8;
mod op_dxyn;
mod op_e;
mod op_f;

use crate::chip_8::emulator::Emulator;
use crate::chip_8::instructions::op_f::op_f;
use anyhow::{anyhow, Result};

pub fn execute_instruction(emulator: &mut Emulator, opcode: u16) -> Result<()> {
    match opcode & 0xF000 {
        0x0000 => op_0::op_0(emulator, opcode)?,

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
            if emulator.v_regs[((opcode & 0x0F00) >> 8) as usize] == (opcode & 0x00FF) as u8 {
                emulator.skip_instruction();
            }
        }

        0x4000 => {
            // 4XNN - Skip next instruction if VX != NN
            if emulator.v_regs[((opcode & 0x0F00) >> 8) as usize] != (opcode & 0x00FF) as u8 {
                emulator.skip_instruction();
            }
        }

        0x5000 => op_5::op_5(emulator, opcode)?,

        0x6000 => {
            // 6XNN - Set VX to NN
            let vx = ((opcode & 0x0F00) >> 8) as usize;
            emulator.v_regs[vx] = (opcode & 0x00FF) as u8;
        }

        0x7000 => {
            // 7XNN - Add NN to VX
            let vx = ((opcode & 0x0F00) >> 8) as usize;
            let nn = opcode & 0x00FF;
            let sum = emulator.v_regs[vx] as u16 + nn;

            if sum > 255 {
                emulator.v_regs[vx] = (sum - 256) as u8;
            } else {
                emulator.v_regs[vx] = sum as u8;
            }
        }

        0x8000 => op_8::op_8(emulator, opcode)?,

        0x9000 => {
            // 9XY0 - Skip next instruction of VX != VY
            if emulator.v_regs[((opcode & 0x0F00) >> 8) as usize]
                != emulator.v_regs[((opcode & 0x00F0) >> 4) as usize]
            {
                emulator.skip_instruction();
            }
        }

        0xA000 => {
            // ANNN - Set I to NNN
            emulator.i_reg = (opcode & 0x0FFF) as usize;
        }

        0xB000 => {
            let reg_value = if !emulator.config.quirks.vx_offset_jump {
                // BNNN - Jump to NNN + V0
                emulator.v_regs[0]
            } else {
                // BXNN - Jump to NNN + VX
                emulator.v_regs[((opcode & 0x0F00) >> 8) as usize]
            };

            emulator.pc = ((opcode & 0x0FFF) + reg_value as u16) as usize;
        }

        0xC000 => {
            // CXNN - Binary AND a random number with NN and set VX to the number
            let vx = ((opcode & 0x0F00) >> 8) as usize;
            emulator.v_regs[vx] = rand::random::<u8>() & (opcode & 0x00FF) as u8;
        }

        0xD000 => op_dxyn::op_dxyn(emulator, opcode)?,

        0xE000 => op_e::op_e(emulator, opcode)?,

        0xF000 => op_f(emulator, opcode)?,

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
