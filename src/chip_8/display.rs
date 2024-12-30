use std::cmp::PartialEq;

#[derive(Clone)]
pub struct Display {
    /// 2D array of booleans, which represent the pixels
    pub pixels: Vec<Vec<bool>>,

    /// Currently active resolution
    pub resolution: Resolution
}

impl Display {
    // Set all pixels to false
    pub fn clear(&mut self) {
        self.pixels = vec![vec![false; self.resolution.width()]; self.resolution.height()];
    }

    /// Set the active resolution
    /// Also clears the display if the resolution has changed
    pub fn set_resolution(&mut self, resolution: Resolution) {
        if resolution == self.resolution {
            return;
        }

        self.resolution = resolution;
        self.pixels = vec![vec![false; resolution.width()]; resolution.height()];
    }
}

impl Default for Display {
    fn default() -> Self {
        let resolution = Resolution::default();

        Self {
            pixels: vec![vec![false; resolution.width()]; resolution.height()],
            resolution,
        }
    }
}

/// SuperChip supports 2 different resolutions:
/// - 64x32 pixels
/// - 128x64 pixels
///
/// The active resolution can be switched using 00FE and 00FF instructions
#[derive(Clone, PartialEq)]
pub enum Resolution {
    Low,
    High,
}

impl Resolution {
    pub fn width(&self) -> usize {
        match self {
            Resolution::Low => 64,
            Resolution::High => 128,
        }
    }

    pub fn height(&self) -> usize {
        match self {
            Resolution::Low => 32,
            Resolution::High => 64,
        }
    }
}

impl Default for Resolution {
    fn default() -> Self {
        Self::Low
    }
}