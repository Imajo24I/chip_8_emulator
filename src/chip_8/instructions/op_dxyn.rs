use crate::chip_8::emulator::Emulator;
use crate::chip_8::instructions::validate_v_reg_index;
use crate::events::Event;

pub fn op_dxyn(emulator: &mut Emulator, opcode: u16) -> Result<(), Event> {
    // DXYN - Draw sprite at coordinate VX, VY with N bytes of sprite data

    let (x, y, height) = get_x_y_height(emulator, opcode)?;

    emulator.v_registers[0xF] = 0;

    for row in 0..height {
        let sprite_data = emulator.memory[emulator.i_register + row];

        for bit in 0..8 {
            if (sprite_data & (0x80 >> bit)) != 0 {
                let x_coord = x + bit;
                let y_coord = y + row;

                if x_coord < 64 || y_coord < 32 {
                    flip_pixel(x_coord, y_coord, emulator);
                }
            }
        }
    }

    Ok(())
}

fn flip_pixel(x_coord: usize, y_coord: usize, emulator: &mut Emulator) {
    if emulator.display[y_coord][x_coord] {
        emulator.v_registers[0xF] = 1;
        emulator.display[y_coord][x_coord] = false;
    } else {
        emulator.display[y_coord][x_coord] = true;
    }
}

fn get_x_y_height(emulator: &mut Emulator, opcode: u16) -> Result<(usize, usize, usize), Event> {
    let vx = ((opcode & 0x0F00) >> 8) as usize;
    validate_v_reg_index(vx, opcode, emulator)?;

    let vy = ((opcode & 0x00F0) >> 4) as usize;
    validate_v_reg_index(vy, opcode, emulator)?;

    let x = (emulator.v_registers[vx] & 63) as usize;
    let y = (emulator.v_registers[vy] & 31) as usize;

    let height = (opcode & 0x000F) as usize;

    Ok((x, y, height))
}
