use std::cmp::PartialEq;

#[derive(Clone)]
pub struct Display {
    /// XO-Chip implemented a second pixel display. This allows for 4 colors, instead of only 2.
    pub planes: [Plane; 2],

    /// Active planes stored as an u8. The least significant bit corresponds to the first plane
    pub active_planes: u8,

    pub resolution: Resolution,
}

impl Display {
    /// For every active plane, run a given closure with the plane as a parameter
    pub fn for_active_plane<F: FnMut(&mut Plane)>(&mut self, mut closure: F) {
        for plane in 0..2 {
            if self.active_planes & (plane as u8 + 1) != 0 {
                closure(&mut self.planes[plane]);
            }
        }

        if self.active_planes & 0b01 == 0b01 {
            closure(&mut self.planes[0]);
        }

        if self.active_planes & 0b10 == 0b10 {
            closure(&mut self.planes[1]);
        }
    }

    /// Set the active resolution
    /// Also clears the display if the resolution has changed
    pub fn set_resolution(&mut self, resolution: Resolution) {
        if resolution == self.resolution {
            return;
        }

        self.resolution = resolution;
        self.planes.iter_mut().for_each(|plane| {
            plane.resolution = self.resolution.clone();
            plane.clear();
        });
    }

    pub fn zip_planes(&mut self) -> Vec<Vec<(bool, bool)>> {
        self.planes[0]
            .pixels
            .clone()
            .into_iter()
            .zip(self.planes[1].pixels.clone().into_iter())
            .map(|(plane1, plane2)| plane1.into_iter().zip(plane2.into_iter()).collect())
            .collect()
    }
}

impl Default for Display {
    fn default() -> Self {
        Self {
            planes: [Plane::default(), Plane::default()],
            active_planes: 0b01,
            resolution: Resolution::default(),
        }
    }
}

#[derive(Clone)]
pub struct Plane {
    pub pixels: Vec<Vec<bool>>,
    pub resolution: Resolution,
}

impl Plane {
    /// Scroll the plane to the right by 4. The 4 leftmost columns will be reset
    pub fn scroll_right(&mut self) {
        for row in self.pixels.iter_mut() {
            row.rotate_right(4);
            for pixel in row.iter_mut().take(4) {
                *pixel = false;
            }
        }
    }

    /// Scroll the plane to the left by 4. The 4 rightmost columns will be reset
    pub fn scroll_left(&mut self) {
        for row in self.pixels.iter_mut() {
            row.rotate_left(4);
            for pixel in row.len() - 4..row.len() {
                row[pixel] = false;
            }
        }
    }

    /// Scroll the plane up by `amount`. The bottom `amount` rows will be reset
    pub fn scroll_up(&mut self, amount: usize) {
        self.pixels.rotate_left(amount);
        for row in self.pixels.len() - amount..self.pixels.len() {
            self.pixels[row] = vec![false; self.resolution.width()];
        }
    }

    /// Scroll the plane down by `amount`. The top `amount` rows will be reset
    pub fn scroll_down(&mut self, amount: usize) {
        self.pixels.rotate_right(amount);
        for row in 0..amount {
            self.pixels[row] = vec![false; self.resolution.width()];
        }
    }

    /// Set all pixels to false
    pub fn clear(&mut self) {
        self.pixels = vec![vec![false; self.resolution.width()]; self.resolution.height()];
    }
}

impl Default for Plane {
    fn default() -> Self {
        let resolution = Resolution::default();

        Self {
            pixels: vec![vec![false; resolution.width()]; resolution.height()],
            resolution,
        }
    }
}

/// SuperChip implements support 2 different resolutions:
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
