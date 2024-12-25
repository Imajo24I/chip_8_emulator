use crate::chip_8::emulator::Emulator;
use crate::chip_8::instructions::unknown_instruction_err;
use anyhow::{anyhow, Result};

pub fn op_f000(emulator: &mut Emulator, opcode: u16) -> Result<()> {
    match opcode & 0x00FF {
        0x0007 => {
            // FX07 - Set VX to value of delay timer
            let vx = ((opcode & 0x0F00) >> 8) as usize;
            emulator.v_registers[vx] = emulator.delay_timer;
        }

        0x0015 => {
            // FX15 - Set delay timer to value of VX
            emulator.delay_timer =
                emulator.v_registers[((opcode & 0x0F00) >> 8) as usize];
        }

        0x0018 => {
            // FX18 - Set sound timer to value of VX
            let currently_playing = emulator.sound_timer > 0;

            emulator.sound_timer =
                emulator.v_registers[((opcode & 0x0F00) >> 8) as usize];

            if currently_playing && emulator.sound_timer == 0 {
                emulator.beeper.pause();
            } else if !currently_playing && emulator.sound_timer > 0 {
                emulator.beeper.play();
            }
        }

        0x001E => {
            // FX1E - Add value of VX to I
            let x = emulator.v_registers[((opcode & 0x0F00) >> 8) as usize];

            if (emulator.i_register + x as usize) > 0x0FFF {
                emulator.v_registers[0xF] = 1;
            } else {
                emulator.v_registers[0xF] = 0;
            }

            emulator.i_register = (emulator.i_register + x as usize) & 0xFFF;
        }

        0x000A => {
            // FX0A - Wait for keypress and store it in VX
            let vx = ((opcode & 0x0F00) >> 8) as usize;

            if let Some(key) = emulator.keypad.get_released_key() {
                emulator.v_registers[vx] = key;
            } else {
                emulator.pc -= 2;
            }
        }

        0x0029 => {
            // FX29 - Set I to location of sprite for hex digit value of VX
            let x = emulator.v_registers[((opcode & 0x0F00) >> 8) as usize];
            emulator.i_register = x as usize * 5;
        }

        0x0033 => {
            // FX33 - Store the binary-coded decimal representation of VX at address I
            let x = emulator.v_registers[((opcode & 0x0F00) >> 8) as usize];

            if emulator.i_register + 2 > 0x0FFF {
                i_reg_out_of_bounds_err(2, opcode, emulator)?;
            }

            emulator.memory[emulator.i_register] = x / 100;
            emulator.memory[emulator.i_register + 1] = (x % 100) / 10;
            emulator.memory[emulator.i_register + 2] = x % 10;
        }

        0x0055 => {
            // FX55 - Store registers V0 to VX in memory starting at address I
            let vx = ((opcode & 0x0F00) >> 8) as usize;

            if emulator.i_register + vx > 0x0FFF {
                i_reg_out_of_bounds_err(vx, opcode, emulator)?;
            }

            for vy in 0..=vx {
                emulator.memory[emulator.i_register + vy] = emulator.v_registers[vy];
            }

            increment_i_quirk(emulator);
        }

        0x0065 => {
            // FX65 - Read registers V0 to VX from memory starting at address I
            let vx = ((opcode & 0x0F00) >> 8) as usize;

            if emulator.i_register + vx > 0x0FFF {
                i_reg_out_of_bounds_err(vx, opcode, emulator)?;
            }

            for vy in 0..=vx {
                emulator.v_registers[vy] = emulator.memory[emulator.i_register + vy];
            }

            increment_i_quirk(emulator);
        }

        _ => unknown_instruction_err(emulator, opcode)?,
    }

    Ok(())
}

fn i_reg_out_of_bounds_err(i_reg_shift: usize, opcode: u16, emulator: &mut Emulator) -> Result<()> {
    Err(anyhow!(
        "I register with value of {} is out of bounds - Instruction {:#06x} is located at memory location {}", emulator.i_register + i_reg_shift, opcode, emulator.pc - 2
    ))
}

fn increment_i_quirk(emulator: &mut Emulator) {
    if emulator.config.quirks.increment_i_reg {
        emulator.i_register += 1;
    }
}
