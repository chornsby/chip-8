/// Stores the current lit state of every pixel on the Chip-8 display
pub struct DisplayState {
    pixels: [[bool; 640]; 320],
}

impl Default for DisplayState {
    fn default() -> Self {
        Self {
            pixels: [[false; 640]; 320],
        }
    }
}

impl DisplayState {
    /// Resets the display to a blank screen
    pub fn clear(&mut self) {
        self.pixels = [[false; 640]; 320];
    }
}
