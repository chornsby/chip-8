pub const HEIGHT: usize = 32;
pub const WIDTH: usize = 64;

/// Stores the current lit state of every pixel on the Chip-8 display
pub struct Display {
    pixels: [[bool; WIDTH]; HEIGHT],
}

impl Default for Display {
    fn default() -> Self {
        Self::new(&[])
    }
}

impl Display {
    /// Creates a new display with some pixels already active
    pub fn new(active: &[(usize, usize)]) -> Self {
        let mut pixels = [[false; WIDTH]; HEIGHT];

        for (x, y) in active {
            pixels[*y][*x] = true;
        }

        Self { pixels }
    }

    /// Resets the display to a blank screen
    pub fn clear(&mut self) {
        self.pixels = [[false; WIDTH]; HEIGHT];
    }

    /// Returns whether the current pixel is active
    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.pixels[y][x]
    }

    /// Flips bits to draw a sprite on the screen
    ///
    /// The sprite is interpreted as a list of rows of bytes where each bit is
    /// XOR'd with the current pixel value to determine the resulting pixel
    /// state.
    ///
    /// Drawing begins at the screen coordinates specified by x and y and will
    /// wrap around the edges of the screen if needed.
    ///
    /// This method returns whether any pixels were erased from the screen as
    /// part of this draw operation.
    pub fn xor_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let mut erased = false;

        for (j, row) in sprite.iter().enumerate() {
            let sprite = row.reverse_bits();

            for i in 0..8 {
                let x = (x + i) % WIDTH;
                let y = (y + j) % HEIGHT;
                let bit = (sprite >> i) % 2 == 1;

                let before = self.pixels[y][x];
                self.pixels[y][x] ^= bit;

                if before && bit {
                    erased = true;
                }
            }
        }

        erased
    }
}
