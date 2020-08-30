/// Stores the current lit state of every pixel on the Chip-8 display
pub struct DisplayState {
    pub pixels: [[bool; 64]; 32],
}

impl Default for DisplayState {
    fn default() -> Self {
        Self {
            pixels: [[false; 64]; 32],
        }
    }
}

impl DisplayState {
    /// Resets the display to a blank screen
    pub fn clear(&mut self) {
        self.pixels = [[false; 64]; 32];
    }
}
