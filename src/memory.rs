pub const PROGRAM_OFFSET: usize = 0x200;

const MEMORY_SIZE: usize = 0x1000;
const DIGITS_OFFSET: usize = 0x000;
const DIGIT_AMOUNT: usize = 16;
const DIGIT_SPRITE_LENGTH: usize = 5;

#[rustfmt::skip]
const DIGITS: [u8; DIGIT_AMOUNT * DIGIT_SPRITE_LENGTH] = [
    // 0
    0b11110000,
    0b10010000,
    0b10010000,
    0b10010000,
    0b11110000,
    // 1
    0b00100000,
    0b01100000,
    0b00100000,
    0b00100000,
    0b01110000,
    // 2
    0b11110000,
    0b00010000,
    0b11110000,
    0b10000000,
    0b11110000,
    // 3
    0b11110000,
    0b00010000,
    0b11110000,
    0b00010000,
    0b11110000,
    // 4
    0b10010000,
    0b10010000,
    0b11110000,
    0b00010000,
    0b00010000,
    // 5
    0b11110000,
    0b10000000,
    0b11110000,
    0b00010000,
    0b11110000,
    // 6
    0b11110000,
    0b10000000,
    0b11110000,
    0b10010000,
    0b11110000,
    // 7
    0b11110000,
    0b00010000,
    0b00100000,
    0b01000000,
    0b01000000,
    // 8
    0b11110000,
    0b10010000,
    0b11110000,
    0b10010000,
    0b11110000,
    // 9
    0b11110000,
    0b10010000,
    0b11110000,
    0b00010000,
    0b11110000,
    // A
    0b11110000,
    0b10010000,
    0b11110000,
    0b10010000,
    0b10010000,
    // B
    0b11100000,
    0b10010000,
    0b11100000,
    0b10010000,
    0b11100000,
    // C
    0b11110000,
    0b10000000,
    0b10000000,
    0b10000000,
    0b11110000,
    // D
    0b11100000,
    0b10010000,
    0b10010000,
    0b10010000,
    0b11100000,
    // E
    0b11110000,
    0b10000000,
    0b11110000,
    0b10000000,
    0b11110000,
    // F
    0b11110000,
    0b10000000,
    0b11110000,
    0b10000000,
    0b10000000,
];

/// Stores the current state of the Chip-8 memory
pub struct Memory {
    memory: [u8; MEMORY_SIZE],
}

impl Memory {
    /// Loads a Chip-8 rom into memory
    ///
    /// The returned Memory struct contains the rom loaded into memory and also
    /// the sprites for the digits that exist in the area specifically reserved
    /// for the interpreter.
    pub fn new(rom: &[u8]) -> Self {
        let mut memory = [0; MEMORY_SIZE];

        for (index, &byte) in DIGITS.iter().enumerate() {
            memory[DIGITS_OFFSET + index] = byte;
        }

        for (index, &byte) in rom.iter().enumerate() {
            memory[PROGRAM_OFFSET + index] = byte;
        }

        Self { memory }
    }

    /// Returns the position in memory for the given digit
    ///
    /// # Panics
    ///
    /// This function will panic if there is no sprite available for the given
    /// digit.
    pub fn calculate_digit_offset(digit: u8) -> usize {
        if (digit as usize) < DIGIT_AMOUNT {
            DIGITS_OFFSET + digit as usize * DIGIT_SPRITE_LENGTH
        } else {
            panic!("No sprite for digit {}", digit)
        }
    }

    /// Returns the byte stored at the given offset
    pub fn get_byte(&self, offset: usize) -> u8 {
        self.memory[offset]
    }

    /// Returns the two-byte instruction stored at the given offset
    pub fn get_instruction(&self, offset: usize) -> u16 {
        let byte_1 = self.memory[offset];
        let byte_2 = self.memory[offset + 1];

        (byte_1 as u16) << 8 | byte_2 as u16
    }

    /// Returns the bytes from the given offset to be interpreted as a sprite
    pub fn get_sprite(&self, offset: usize, n: usize) -> &[u8] {
        &self.memory[offset..offset + n]
    }

    /// Replaces the byte at the given offset with the new value
    pub fn set_byte(&mut self, offset: usize, byte: u8) {
        self.memory[offset] = byte;
    }
}
