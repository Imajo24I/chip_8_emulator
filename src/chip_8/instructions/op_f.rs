use crate::chip_8::emulator::Emulator;
use crate::chip_8::instructions::unknown_instruction_err;
use anyhow::{anyhow, Result};

/// Execute instructions which start with F
pub fn op_f(emulator: &mut Emulator, opcode: u16) -> Result<()> {
    match opcode & 0x00FF {
        0x0000 => {
            // XO-Chip Instruction
            // F000 - Set I to the next 2 bytes of memory at PC
            emulator.i_reg = (emulator.memory[emulator.pc] as usize) << 8
                | emulator.memory[emulator.pc + 1] as usize;

            // Skip next 2 bytes, since they are used by this instruction
            emulator.pc += 2;
        }

        0x0001 => {
            // XO-Chip Instruction
            // FN01 - Select active planes
            emulator.display.active_planes = ((opcode & 0x0F00) >> 8) as u8;
        }

        0x0002 => {
            // XO-Chip Instruction
            // F002 - Store 16 bytes of memory starting at I into audio pattern buffer
            // TODO: Implement this
        }

        0x003A => {
            // XO-Chip Instruction
            // FX3A - Set audio pattern playback rate to 4000*2^((VX-64)/48)
            // TODO: Implement this
        }

        0x0007 => {
            // FX07 - Set VX to value of delay timer
            let vx = ((opcode & 0x0F00) >> 8) as usize;
            emulator.v_regs[vx] = emulator.delay_timer;
        }

        0x0015 => {
            // FX15 - Set delay timer to value of VX
            emulator.delay_timer = emulator.v_regs[((opcode & 0x0F00) >> 8) as usize];
        }

        0x0018 => {
            // FX18 - Set sound timer to value of VX
            let currently_playing = emulator.sound_timer > 0;

            emulator.sound_timer = emulator.v_regs[((opcode & 0x0F00) >> 8) as usize];

            if currently_playing && emulator.sound_timer == 0 {
                emulator.beeper.pause();
            } else if !currently_playing && emulator.sound_timer > 0 {
                emulator.beeper.play();
            }
        }

        0x001E => {
            // FX1E - Add value of VX to I
            emulator.i_reg =
                emulator.i_reg + emulator.v_regs[((opcode & 0x0F00) >> 8) as usize] as usize;
        }

        0x000A => {
            // FX0A - Wait for keypress and store it in VX
            let vx = ((opcode & 0x0F00) >> 8) as usize;

            if let Some(key) = emulator.keypad.get_released_key() {
                emulator.v_regs[vx] = key;
            } else {
                emulator.pc -= 2;
            }
        }

        0x0029 => {
            // FX29 - Set I to location of small sprite for hex digit value of VX
            let x = emulator.v_regs[((opcode & 0x0F00) >> 8) as usize];
            emulator.i_reg = x as usize * 5;
        }

        0x0030 => {
            // SuperChip Instruction
            // FX30 - Set I to location of large sprite for hex value of VX
            let x = emulator.v_regs[((opcode & 0x0F00) >> 8) as usize];
            emulator.i_reg = 80 + (x as usize * 10);
        }

        0x0033 => {
            // FX33 - Store the binary-coded decimal representation of VX at address I
            let x = emulator.v_regs[((opcode & 0x0F00) >> 8) as usize];

            if emulator.i_reg + 2 > emulator.config.memory_size {
                i_reg_out_of_bounds_err(2, opcode, emulator)?;
            }

            emulator.memory[emulator.i_reg] = x / 100;
            emulator.memory[emulator.i_reg + 1] = (x % 100) / 10;
            emulator.memory[emulator.i_reg + 2] = x % 10;
        }

        0x0055 => {
            // FX55 - Store registers V0 to VX in memory starting at address I
            let vx = ((opcode & 0x0F00) >> 8) as usize;

            if emulator.i_reg + vx > emulator.config.memory_size {
                i_reg_out_of_bounds_err(vx, opcode, emulator)?;
            }

            for vy in 0..=vx {
                emulator.memory[emulator.i_reg + vy] = emulator.v_regs[vy];
            }

            increment_i_quirk(emulator, vx);
        }

        0x0065 => {
            // FX65 - Read registers V0 to VX from memory starting at address I
            let vx = ((opcode & 0x0F00) >> 8) as usize;

            if emulator.i_reg + vx > emulator.config.memory_size {
                i_reg_out_of_bounds_err(vx, opcode, emulator)?;
            }

            for vy in 0..=vx {
                emulator.v_regs[vy] = emulator.memory[emulator.i_reg + vy];
            }

            increment_i_quirk(emulator, vx);
        }

        0x0075 => {
            // SuperChip Instruction
            // FX75 - Store V0 - VX into flag registers
            let vx = ((opcode & 0x0F00) >> 8) as usize;

            for reg in 0..vx {
                emulator.f_regs[reg] = emulator.v_regs[reg];
            }
        }

        0x0085 => {
            // SuperChip Instruction
            // FX85 - Load V0 - VX from flag registers
            let vx = ((opcode & 0x0F00) >> 8) as usize;

            for reg in 0..vx {
                emulator.v_regs[reg] = emulator.f_regs[reg];
            }
        }

        _ => unknown_instruction_err(emulator, opcode)?,
    }

    Ok(())
}

fn i_reg_out_of_bounds_err(i_reg_shift: usize, opcode: u16, emulator: &mut Emulator) -> Result<()> {
    Err(anyhow!(
        "I register with value of {} is out of bounds - Instruction {:#06x} is located at memory location {}", emulator.i_reg + i_reg_shift, opcode, emulator.pc - 2
    ))
}

fn increment_i_quirk(emulator: &mut Emulator, vx: usize) {
    if emulator.config.quirks.increment_i_reg {
        // Increment I register
        // & 0xFFFF is used to ensure that the I Register stays in the 16 bit range
        emulator.i_reg = (emulator.i_reg + 1 + vx) & 0xFFFF;
    }
}
