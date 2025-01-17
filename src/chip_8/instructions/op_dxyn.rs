use crate::chip_8::emulator::Emulator;
use crate::chip_8::instructions::memory_index_out_of_bounds_err;
use anyhow::Result;

/// Execute DXYN Instruction
pub fn op_dxyn(emulator: &mut Emulator, opcode: u16) -> Result<()> {
    // DXYN - Draw sprite at coordinate VX, VY with N bytes of sprite data

    let display_width = emulator.display.resolution.width();
    let display_height = emulator.display.resolution.height();

    let starting_x = emulator.v_regs[((opcode & 0x0F00) >> 8) as usize] as usize % display_width;
    let starting_y = emulator.v_regs[((opcode & 0x00F0) >> 4) as usize] as usize % display_height;
    let height = (opcode & 0x000F) as usize;
    let mut i = emulator.i_reg;

    let sprite_width = if height == 0 { 16 } else { 8 };
    let sprite_height = if height == 0 { 16 } else { height };

    emulator.v_regs[0xF] = 0;

    for layer in 0..2 {
        // Check if current layer is selected
        if emulator.display.active_planes & (layer as u8 + 1) == 0 {
            continue;
        }

        for row in 0..sprite_height {
            let sprite_data = if height == 0 {
                if i + row * 2 + 1 >= emulator.config.memory_size {
                    return memory_index_out_of_bounds_err(i + row * 2 + 1, emulator, opcode);
                }

                ((emulator.memory[i + row * 2] as u16) << 8)
                    | (emulator.memory[i + row * 2 + 1] as u16)
            } else {
                if i + row >= emulator.config.memory_size {
                    return memory_index_out_of_bounds_err(i + row, emulator, opcode);
                }

                emulator.memory[i + row] as u16
            };

            for index in 0..sprite_width {
                let offset = if height == 0 { 15 - index } else { 7 - index };
                let bit = (sprite_data & (1 << offset)) >> offset;

                if bit != 0 {
                    let (x, y) = if emulator.config.quirks.wrap_sprites {
                        (
                            (starting_x + index) % display_width,
                            (starting_y + row) % display_height,
                        )
                    } else {
                        (starting_x + index, starting_y + row)
                    };

                    if x < display_width && y < display_height {
                        let pixel = &mut emulator.display.planes[layer].pixels[y][x];

                        if *pixel {
                            emulator.v_regs[0xF] = 1;
                            *pixel = false;
                        } else {
                            *pixel = true;
                        }
                    }
                }
            }
        }

        i += if height == 0 { 32 } else { height };
    }

    Ok(())
}
