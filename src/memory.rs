pub const PROGRAM_OFFSET: usize = 0x200;

const MEMORY_SIZE: usize = 0x1000;
const DIGITS_OFFSET: usize = 0x000;
const DIGIT_SPRITE_LENGTH: usize = 5;

#[rustfmt::skip]
const DIGITS: [u8; 16 * DIGIT_SPRITE_LENGTH] = [
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

pub struct Memory {
    memory: [u8; MEMORY_SIZE],
}

impl Memory {
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

    pub fn calculate_digit_offset(digit: u8) -> usize {
        DIGITS_OFFSET + digit as usize * DIGIT_SPRITE_LENGTH
    }

    pub fn get_byte(&self, offset: usize) -> u8 {
        self.memory[offset]
    }

    pub fn get_instruction(&self, offset: usize) -> u16 {
        let byte_1 = self.memory[offset];
        let byte_2 = self.memory[offset + 1];

        (byte_1 as u16) << 8 | byte_2 as u16
    }

    pub fn get_sprite(&self, offset: usize, n: usize) -> &[u8] {
        &self.memory[offset..offset + n]
    }

    pub fn set_byte(&mut self, offset: usize, byte: u8) {
        self.memory[offset] = byte;
    }
}
