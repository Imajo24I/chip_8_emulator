use crate::chip_8::config::Quirks;
use crate::chip_8::display::Resolution;
use crate::chip_8::emulator::Emulator;
use anyhow::Result;

/// Execute instructions which start with D
pub fn op_d(emulator: &mut Emulator, opcode: u16) -> Result<()> {
    let resolution = emulator.display.resolution.clone();
    let display_width = resolution.width();
    let display_height = resolution.height();

    match opcode & 0x000F {
        0x0000 => {
            // SuperChip Instruction
            // DXY0 - Draw 16x16 sprite

            // Discard height since it is always 16
            let (starting_x, starting_y, _) = get_starting_x_y_height(emulator, opcode)?;

            for row in 0..16 {
                // Combine two bytes into one u16
                let sprite_data = ((emulator.memory[emulator.i_reg + row * 2] as u16) << 8)
                    | (emulator.memory[emulator.i_reg + row * 2 + 1] as u16);

                for bit in 0..16 {
                    // Check if bit isn't 0
                    if (sprite_data & (0x8000 >> bit)) != 0 {
                        let (x, y) = calculate_coord(
                            &emulator.config.quirks,
                            &resolution,
                            (starting_x, starting_y),
                            (bit, row),
                        );

                        if x < display_width && y < display_height {
                            flip_pixel(x, y, emulator);
                        }
                    }
                }
            }
        }

        _ => {
            // DXYN - Draw sprite at coordinate VX, VY with N bytes of sprite data

            let (starting_x, starting_y, height) = get_starting_x_y_height(emulator, opcode)?;
            emulator.v_regs[0xF] = 0;

            for row in 0..height {
                let sprite_data = emulator.memory[emulator.i_reg + row];

                for bit in 0..8 {
                    // Check if bit isn't 0
                    if (sprite_data & (0x80 >> bit)) != 0 {
                        let (x, y) = calculate_coord(
                            &emulator.config.quirks,
                            &resolution,
                            (starting_x, starting_y),
                            (bit, row),
                        );

                        if x < display_width && y < display_height {
                            flip_pixel(x, y, emulator);
                        }
                    }
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

fn get_starting_x_y_height(emulator: &mut Emulator, opcode: u16) -> Result<(usize, usize, usize)> {
    let resolution = &emulator.display.resolution;

    let x = emulator.v_regs[((opcode & 0x0F00) >> 8) as usize] as usize % resolution.width();
    let y = emulator.v_regs[((opcode & 0x00F0) >> 4) as usize] as usize % resolution.height();
    let height = (opcode & 0x000F) as usize;

    Ok((x, y, height))
}

fn calculate_coord(
    quirks: &Quirks,
    resolution: &Resolution,
    starting_coords: (usize, usize),
    offsets: (usize, usize),
) -> (usize, usize) {
    let (starting_x, starting_y) = starting_coords;
    let (x_offset, y_offset) = offsets;

    if quirks.wrap_sprites {
        (
            (starting_x + x_offset) % resolution.width(),
            (starting_y + y_offset) % resolution.height(),
        )
    } else {
        (starting_x + x_offset, starting_y + y_offset)
    }
}
