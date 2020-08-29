use crate::keyboard::KeyboardState;
use bevy::prelude::*;

const MEMORY_SIZE: usize = 0x1000;
const PROGRAM_OFFSET: usize = 0x200;

pub struct Emulator {
    memory: [u8; MEMORY_SIZE],
    registers: [u8; 16],
    i: u16,
    delay_timer: u8,
    sound_timer: u8,
    program_counter: usize,
    stack: Vec<usize>,
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
            delay_timer: 0,
            sound_timer: 0,
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

        self.delay_timer = self.delay_timer.saturating_sub(1);
        self.sound_timer = self.sound_timer.saturating_sub(1);

        self.program_counter = match instruction {
            0x1000..=0x1FFF => self.jp(instruction),
            0x2000..=0x2FFF => self.call(instruction),
            0x6000..=0x6FFF => self.ld_v(instruction),
            0x7000..=0x7FFF => self.add_v(instruction),
            0x8000..=0x8FFF => match instruction & 0xF {
                0x3 => self.xor_v_v(instruction),
                _ => panic!("Unknown instruction 0x{:X}", instruction),
            },
            0xA000..=0xAFFF => self.ld_i(instruction),
            0xF000..=0xFFFF => match instruction & 0xFF {
                0x1E => self.add_i_v(instruction),
                0x55 => self.ld_i_v(instruction),
                0x65 => self.ld_v_i(instruction),
                _ => panic!("Unknown instruction 0x{:X}", instruction),
            },
            _ => panic!("Unknown instruction 0x{:X}", instruction),
        }
    }

    /// Jumps the program counter to nnn (0x1nnn)
    fn jp(&self, instruction: u16) -> usize {
        let addr = instruction & 0xFFF;
        addr as usize
    }

    /// Calls subroutine at nnn (0x2nnn)
    fn call(&mut self, instruction: u16) -> usize {
        let addr = instruction & 0xFFF;

        self.stack.push(self.program_counter);
        addr as usize
    }

    /// Loads Vx to kk (0x6xkk)
    fn ld_v(&mut self, instruction: u16) -> usize {
        let vx = (instruction >> 8) & 0xF;
        let byte = (instruction & 0xFF) as u8;

        self.registers[vx as usize] = byte;
        self.program_counter + 2
    }

    /// Adds kk to Vx (0x7xkk)
    fn add_v(&mut self, instruction: u16) -> usize {
        let vx = (instruction >> 8) & 0xF;
        let byte = (instruction & 0xFF) as u8;

        self.registers[vx as usize] = self.registers[vx as usize].wrapping_add(byte);
        self.program_counter + 2
    }

    /// Stores xor of Vx and Vy in Vx (0x8xy3)
    fn xor_v_v(&mut self, instruction: u16) -> usize {
        let vx = (instruction >> 8) & 0xF;
        let vy = (instruction >> 4) & 0xF;

        self.registers[vx as usize] ^= self.registers[vy as usize];
        self.program_counter + 2
    }

    /// Loads Vi to nnn (0xAnnn)
    fn ld_i(&mut self, instruction: u16) -> usize {
        let addr = instruction & 0xFFF;

        self.i = addr;
        self.program_counter + 2
    }

    /// Adds Vx to Vi (0xFx1E)
    fn add_i_v(&mut self, instruction: u16) -> usize {
        let vx = (instruction >> 8) & 0xF;

        self.i = self.i.wrapping_add(self.registers[vx as usize] as u16);
        self.program_counter + 2
    }

    /// Loads [V0, Vx] to memory starting at Vi (0xFx55)
    fn ld_i_v(&mut self, instruction: u16) -> usize {
        let vx = (instruction >> 8) & 0xF;

        for index in 0..=vx {
            self.memory[(self.i + index) as usize] = self.registers[index as usize]
        }

        self.program_counter + 2
    }

    /// Loads memory starting at Vi to [V0, Vx] (0xFx65)
    fn ld_v_i(&mut self, instruction: u16) -> usize {
        let vx = (instruction >> 8) & 0xF;

        for index in 0..=vx {
            self.registers[index as usize] = self.memory[(self.i + index) as usize]
        }

        self.program_counter + 2
    }
}

pub fn emulator_system(mut emulator: ResMut<Emulator>, keyboard: Res<KeyboardState>) {
    emulator.tick(&keyboard);
}
