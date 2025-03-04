use std::path::PathBuf;

#[derive(Clone)]
pub struct Config {
    pub instructions_per_frame: u32,
    pub filepath: Option<PathBuf>,
    pub quirks: Quirks,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            instructions_per_frame: 10,
            filepath: None,
            quirks: Quirks::default(),
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
