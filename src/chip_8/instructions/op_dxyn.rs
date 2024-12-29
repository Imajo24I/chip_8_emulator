use crate::chip_8::emulator::Emulator;
use anyhow::Result;

pub fn op_dxyn(emulator: &mut Emulator, opcode: u16) -> Result<()> {
    // DXYN - Draw sprite at coordinate VX, VY with N bytes of sprite data
    let resolution = &emulator.display.resolution;
    let display_width = resolution.width();
    let display_height = resolution.height();

    let (starting_x, starting_y, height) = get_x_y_height(emulator, opcode)?;
    emulator.v_regs[0xF] = 0;

    for row in 0..height {
        let sprite_data = emulator.memory[emulator.i_reg + row];

        for bit in 0..8 {
            if (sprite_data & (0x80 >> bit)) != 0 {
                let (x, y) = if emulator.config.quirks.wrap_sprites {
                    ((starting_x + bit) % display_width, (starting_y + row) % display_height)
                } else {
                    (starting_x + bit, starting_y + row)
                };

                if x < display_width && y < display_height {
                    flip_pixel(x, y, emulator);
                }
            }
        }
    }

    Ok(())
}

fn flip_pixel(x_coord: usize, y_coord: usize, emulator: &mut Emulator) {
    let pixel = &mut emulator.display.pixels[y_coord][x_coord];

    if *pixel {
        emulator.v_regs[0xF] = 1;
        *pixel = false;
    } else {
        *pixel = true;
    }
}

fn get_x_y_height(emulator: &mut Emulator, opcode: u16) -> Result<(usize, usize, usize)> {
    let resolution = &emulator.display.resolution;

    let x = emulator.v_regs[((opcode & 0x0F00) >> 8) as usize] as usize % resolution.width();
    let y = emulator.v_regs[((opcode & 0x00F0) >> 4) as usize] as usize % resolution.height();
    let height = (opcode & 0x000F) as usize;

    Ok((x, y, height))
}
