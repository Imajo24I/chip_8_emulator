use crate::chip_8::emulator::Emulator;
use crate::chip_8::instructions::{get_v_reg_value, unknown_instruction_err};
use crate::chip_8::keypad::Keypad;
use crate::events::Event;

pub fn op_e000(emulator: &mut Emulator, opcode: u16) -> Result<(), Event> {
    match opcode & 0x00FF {
        0x009E => {
            // EX9E - Skip next instruction if key with the value of VX is pressed
            let key = get_v_reg_value(((opcode & 0x0F00) >> 8) as usize, opcode, emulator)?;
            Keypad::is_key_valid(key, opcode, emulator)?;

            if emulator.keypad.is_key_pressed(key as usize) {
                emulator.pc += 2;
            }
        }

        0x00A1 => {
            // EXA1 - Skip next instruction if key with the value of VX is not pressed
            let key = get_v_reg_value(((opcode & 0x0F00) >> 8) as usize, opcode, emulator)?;
            Keypad::is_key_valid(key, opcode, emulator)?;

            if !emulator.keypad.is_key_pressed(key as usize) {
                emulator.pc += 2;
            }
        }

        _ => unknown_instruction_err(emulator, opcode)?,
    }

    Ok(())
}
