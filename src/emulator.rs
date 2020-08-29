use crate::keyboard::KeyboardState;
use bevy::prelude::*;

const MEMORY_SIZE: usize = 0x1000;
const PROGRAM_OFFSET: usize = 0x200;

pub struct Emulator {
    memory: [u8; MEMORY_SIZE],
    registers: [u8; 16],
    i: u16,
    delay: u8,
    sound: u8,
    program_counter: usize,
    stack: Vec<u16>,
}

impl Emulator {
    pub fn new(rom: &[u8]) -> Self {
        let mut memory = [0; MEMORY_SIZE];

        for (index, &byte) in rom.iter().enumerate() {
            memory[PROGRAM_OFFSET + index] = byte;
        }

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

    pub fn tick(&mut self, _keyboard: &KeyboardState) {
        let instruction = {
            let byte_1 = self.memory[self.program_counter];
            let byte_2 = self.memory[self.program_counter + 1];

            (byte_1 as u16) << 8 | byte_2 as u16
        };

        println!("0x{:X}", instruction);

        match instruction {
            0x1000..=0x1FFF => {
                let addr = instruction & 0xFFF;
                self.program_counter = addr as usize;
                return;
            }
            _ => panic!("Unknown instruction 0x{:X}", instruction),
        }

        self.program_counter += 2;
    }
}

pub fn emulator_system(mut emulator: ResMut<Emulator>, keyboard: Res<KeyboardState>) {
    emulator.tick(&keyboard);
}
