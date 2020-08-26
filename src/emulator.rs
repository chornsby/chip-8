use crate::memory::{Memory, PROGRAM_OFFSET};
use crate::rom::Rom;

pub struct Emulator {
    memory: Memory,
    registers: [u8; 16],
    i: u16,
    delay: u8,
    sound: u8,
    program_counter: usize,
    stack: Vec<u16>,
}

impl Emulator {
    pub fn new(rom: &Rom) -> Self {
        let mut memory = Memory::default();
        memory.load_rom(rom);

        Self {
            memory,
            registers: [0; 16],
            i: 0,
            delay: 0,
            sound: 0,
            program_counter: PROGRAM_OFFSET,
            stack: vec![],
        }
    }

    pub fn tick(&mut self) {
        let byte_1 = self.memory.0[self.program_counter];
        let byte_2 = self.memory.0[self.program_counter + 1];

        match (byte_1, byte_2) {
            _ => {}
        }

        self.program_counter += 2;
    }
}
