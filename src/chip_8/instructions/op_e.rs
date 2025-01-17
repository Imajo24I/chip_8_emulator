use crate::chip_8::emulator::Emulator;
use crate::chip_8::instructions::unknown_instruction_err;
use anyhow::Result;

/// Execute instructions which start with E
pub fn op_e(emulator: &mut Emulator, opcode: u16) -> Result<()> {
    let key = emulator.v_regs[((opcode & 0x0F00) >> 8) as usize] & 0xF;

    match opcode & 0x00FF {
        0x009E => {
            // EX9E - Skip next instruction if key with the value of VX is pressed
            if emulator.keypad.is_key_pressed(key as usize) {
                emulator.skip_instruction();
            }
        }

        0x00A1 => {
            // EXA1 - Skip next instruction if key with the value of VX is not pressed
            if !emulator.keypad.is_key_pressed(key as usize) {
                emulator.skip_instruction();
            }
        }

        _ => unknown_instruction_err(emulator, opcode)?,
    }

    Ok(())
}
