/// One of the hex keys on the Chip-8 keypad
#[derive(Copy, Clone)]
pub enum Key {
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    A,
    B,
    C,
    D,
    E,
    F,
}

impl From<u8> for Key {
    /// Converts a byte into a Key
    fn from(value: u8) -> Self {
        match value {
            0x0 => Key::Num0,
            0x1 => Key::Num1,
            0x2 => Key::Num2,
            0x3 => Key::Num3,
            0x4 => Key::Num4,
            0x5 => Key::Num5,
            0x6 => Key::Num6,
            0x7 => Key::Num7,
            0x8 => Key::Num8,
            0x9 => Key::Num9,
            0xA => Key::A,
            0xB => Key::B,
            0xC => Key::C,
            0xD => Key::D,
            0xE => Key::E,
            0xF => Key::F,
            _ => panic!("Unsupported key {}", value),
        }
    }
}

/// Stores the current pressed state of the Chip-8 hex keys
#[derive(Default)]
pub struct Keyboard {
    pressed: [bool; 16],
}

impl Keyboard {
    /// Returns an iterator over all possible Keys
    pub fn keys() -> impl Iterator<Item = Key> {
        [
            Key::Num0,
            Key::Num1,
            Key::Num2,
            Key::Num3,
            Key::Num4,
            Key::Num5,
            Key::Num6,
            Key::Num7,
            Key::Num8,
            Key::Num9,
            Key::A,
            Key::B,
            Key::C,
            Key::D,
            Key::E,
            Key::F,
        ]
        .iter()
        .copied()
    }
    /// Returns whether the given key is currently pressed
    pub fn is_pressed(&self, key: &Key) -> bool {
        self.pressed[*key as usize]
    }

    /// Releases the given key on the keyboard
    pub fn release(&mut self, key: &Key) {
        self.pressed[*key as usize] = false;
    }

    /// Presses the given key on the keyboard
    pub fn press(&mut self, key: &Key) {
        self.pressed[*key as usize] = true;
    }
}
