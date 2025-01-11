use crate::chip_8::emulator::Emulator;
use crate::chip_8::instructions::unknown_instruction_err;
use anyhow::Result;

/// Execute instructions which start with 5
pub fn op_5(emulator: &mut Emulator, opcode: u16) -> Result<()> {
    let vx = ((opcode & 0x0F00) >> 8) as usize;
    let vy = ((opcode & 0x00F0) >> 4) as usize;

    match opcode & 0x000F {
        0x0000 => {
            // 5XY0 - Skip next instruction of VX == VY
            if emulator.v_regs[vx] == emulator.v_regs[vy] {
                emulator.skip_instruction();
            }
        }

        0x0002 => {
            // XO-Chip Instruction
            // 5XY2 - Save registers VX - VY to memory starting at I
            let diff = vx.abs_diff(vy);

            for i in 0..=diff {
                let reg = if vx < vy { vx + i } else { vx - i };
                emulator.memory[emulator.i_reg + i] = emulator.v_regs[reg];
            }
        }

        0x0003 => {
            // XO-Chip Instruction
            // 5XY3 - Load registers VX - VY from memory starting at I
            let diff = vx.abs_diff(vy);

            for i in 0..=diff {
                let reg = if vx < vy { vx + i } else { vx - i };
                emulator.v_regs[reg] = emulator.memory[emulator.i_reg + i];
            }
        }

        _ => unknown_instruction_err(emulator, opcode)?,
    }

    Ok(())
}
