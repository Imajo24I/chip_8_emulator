use std::path::PathBuf;

#[derive(Clone)]
pub struct Config {
    pub instructions_per_frame: u32,
    pub emulation_paused: bool,
    pub filepath: Option<PathBuf>,
    pub quirks: Quirks,
    pub memory_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            instructions_per_frame: 10,
            emulation_paused: false,
            filepath: None,
            quirks: Quirks::default(),
            memory_size: 4096,
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct Quirks {
    pub vf_reset: bool,
    pub increment_i_reg: bool,
    pub vx_offset_jump: bool,
    pub shift_vx_directly: bool,
    pub wrap_sprites: bool,
}
