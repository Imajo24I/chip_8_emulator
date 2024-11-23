use crate::emulator::emulator::Emulator;
use crate::emulator::instructions::{get_v_reg_value, unknown_instruction_err};
use crate::events::Event;
use eframe::egui::InputState;

pub fn x_e000(emulator: &mut Emulator, opcode: u16, input_state: &InputState) -> Result<(), Event> {
    match opcode & 0x00FF {
        0x009E => {
            // EX9E - Skip next instruction if key with the value of VX is pressed
            let key = get_v_reg_value(((opcode & 0x0F00) >> 8) as usize, opcode, emulator)?;

            if emulator.is_key_pressed(key, input_state, opcode)? {
                emulator.pc += 2;
            }
        }

        0x00A1 => {
            // EXA1 - Skip next instruction if key with the value of VX is not pressed
            let key = get_v_reg_value(((opcode & 0x0F00) >> 8) as usize, opcode, emulator)?;

            if !emulator.is_key_pressed(key, input_state, opcode)? {
                emulator.pc += 2;
            }
        }

        _ => unknown_instruction_err(emulator, opcode)?,
    }

    Ok(())
}
