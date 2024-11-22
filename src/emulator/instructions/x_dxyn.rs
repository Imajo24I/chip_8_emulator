use crate::emulator::emulator::Emulator;
use crate::emulator::instructions::validate_v_reg_index;
use crate::events::Event;

pub fn x_dxyn(emulator: &mut Emulator, opcode: u16) -> Result<(), Event> {
    // DXYN - Draw sprite at coordinate VX, VY with N bytes of sprite data

    let vx = ((opcode & 0x0F00) >> 8) as usize;
    let vy = ((opcode & 0x00F0) >> 4) as usize;

    for v in [vx, vy].iter() {
        validate_v_reg_index(*v, opcode, emulator)?;
    }

    let x = (emulator.v_registers[vx] & 63) as usize;
    let y = (emulator.v_registers[vy] & 31) as usize;

    emulator.v_registers[0xF] = 0;


    let n = (opcode & 0x000F) as usize;

    for row in 0..n {
        let sprite_data = emulator.memory[emulator.i_register + row];
        let mut bit_x = x;
        for bit in (0..8).rev() {
            let current_bit = (sprite_data >> bit) & 1;
            emulator.display[y + row][bit_x] = current_bit == 1;

            if current_bit == 1 {
                emulator.v_registers[0xF] = 1;
            }

            bit_x += 1;
            if bit_x == 64 {
                break;
            }
        }

        if y + row == 32 {
            break;
        }
    }

    Ok(())
}