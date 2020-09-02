use crate::display::Display;
use crate::keyboard::Keyboard;

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

    pub fn decrement_timers(&mut self) {
        self.delay_timer = self.delay_timer.saturating_sub(1);
        self.sound_timer = self.sound_timer.saturating_sub(1);
    }

    pub fn tick(&mut self, display: &mut Display, _keyboard: &Keyboard) {
        let instruction = {
            let byte_1 = self.memory[self.program_counter];
            let byte_2 = self.memory[self.program_counter + 1];

            (byte_1 as u16) << 8 | byte_2 as u16
        };

        println!("{}: {:#04X}", self.program_counter, instruction);

        self.program_counter = match instruction {
            0x00E0 => self.cls(display),
            0x00EE => self.ret(),
            0x1000..=0x1FFF => self.jp(instruction),
            0x2000..=0x2FFF => self.call(instruction),
            0x3000..=0x3FFF => self.se_v(instruction),
            0x4000..=0x4FFF => self.sne_v(instruction),
            0x6000..=0x6FFF => self.ld_v(instruction),
            0x7000..=0x7FFF => self.add_v(instruction),
            0x8000..=0x8FFF => match instruction & 0xF {
                0x0 => self.ld_v_v(instruction),
                0x1 => self.or_v_v(instruction),
                0x2 => self.and_v_v(instruction),
                0x3 => self.xor_v_v(instruction),
                0x4 => self.add_v_v(instruction),
                0x6 => self.shr_v_v(instruction),
                0xE => self.shl_v_v(instruction),
                _ => panic!("Unknown instruction 0x{:X}", instruction),
            },
            0xA000..=0xAFFF => self.ld_i(instruction),
            0xD000..=0xDFFF => self.drw(instruction, display),
            0xF000..=0xFFFF => match instruction & 0xFF {
                0x1E => self.add_i_v(instruction),
                0x55 => self.ld_i_v(instruction),
                0x65 => self.ld_v_i(instruction),
                _ => panic!("Unknown instruction 0x{:X}", instruction),
            },
            _ => panic!("Unknown instruction 0x{:X}", instruction),
        }
    }

    /// Clears the display
    fn cls(&self, display: &mut Display) -> usize {
        display.clear();
        self.program_counter + 2
    }

    /// Returns from a subroutine
    fn ret(&mut self) -> usize {
        self.stack.pop().expect("No subroutine to return from") + 2
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

    /// Skips an instruction if Vx == kk (0x3xkk)
    fn se_v(&mut self, instruction: u16) -> usize {
        let vx = instruction >> 8 & 0xF;
        let byte = (instruction & 0xFF) as u8;

        if self.registers[vx as usize] == byte {
            self.program_counter + 4
        } else {
            self.program_counter + 2
        }
    }

    /// Skips an instruction if Vx != kk (0x4xkk)
    fn sne_v(&mut self, instruction: u16) -> usize {
        let vx = instruction >> 8 & 0xF;
        let byte = (instruction & 0xFF) as u8;

        if self.registers[vx as usize] == byte {
            self.program_counter + 2
        } else {
            self.program_counter + 4
        }
    }

    /// Loads kk to Vx (0x6xkk)
    fn ld_v(&mut self, instruction: u16) -> usize {
        let vx = instruction >> 8 & 0xF;
        let byte = (instruction & 0xFF) as u8;

        self.registers[vx as usize] = byte;
        self.program_counter + 2
    }

    /// Adds kk to Vx (0x7xkk)
    fn add_v(&mut self, instruction: u16) -> usize {
        let vx = instruction >> 8 & 0xF;
        let byte = (instruction & 0xFF) as u8;

        self.registers[vx as usize] = self.registers[vx as usize].wrapping_add(byte);
        self.program_counter + 2
    }

    /// Sets Vy to Vx (0x8xy0)
    fn ld_v_v(&mut self, instruction: u16) -> usize {
        let vx = instruction >> 8 & 0xF;
        let vy = instruction >> 4 & 0xF;

        self.registers[vx as usize] = self.registers[vy as usize];
        self.program_counter + 2
    }

    /// Stores bitwise OR of Vx and Vy to Vx (0x8xy1)
    fn or_v_v(&mut self, instruction: u16) -> usize {
        let vx = instruction >> 8 & 0xF;
        let vy = instruction >> 4 & 0xF;

        self.registers[vx as usize] |= self.registers[vy as usize];
        self.program_counter + 2
    }

    /// Stores bitwise AND of Vx and Vy in Vx (0x8xy2)
    fn and_v_v(&mut self, instruction: u16) -> usize {
        let vx = instruction >> 8 & 0xF;
        let vy = instruction >> 4 & 0xF;

        self.registers[vx as usize] &= self.registers[vy as usize];
        self.program_counter + 2
    }

    /// Stores bitwise XOR of Vx and Vy in Vx (0x8xy3)
    fn xor_v_v(&mut self, instruction: u16) -> usize {
        let vx = instruction >> 8 & 0xF;
        let vy = instruction >> 4 & 0xF;

        self.registers[vx as usize] ^= self.registers[vy as usize];
        self.program_counter + 2
    }

    /// Adds Vx and Vy with carry (0x8xy4)
    fn add_v_v(&mut self, instruction: u16) -> usize {
        let vx = instruction >> 8 & 0xF;
        let vy = instruction >> 4 & 0xF;

        let (value, carry) =
            self.registers[vx as usize].overflowing_add(self.registers[vy as usize]);

        self.registers[0xF] = carry as u8;
        self.registers[vx as usize] = value;
        self.program_counter + 2
    }

    /// Shifts Vx to the right with carry (0x8xy6)
    fn shr_v_v(&mut self, instruction: u16) -> usize {
        let vx = instruction >> 8 & 0xF;

        self.registers[0xF] = self.registers[vx as usize] % 2;
        self.registers[vx as usize] >>= 1;
        self.program_counter + 2
    }

    /// Shifts Vx to the left with carry (0x8xyE)
    fn shl_v_v(&mut self, instruction: u16) -> usize {
        let vx = instruction >> 8 & 0xF;

        self.registers[0xF] = (0b10000000 <= self.registers[vx as usize]) as u8;
        self.registers[vx as usize] <<= 1;
        self.program_counter + 2
    }

    /// Loads nnn to Vi (0xAnnn)
    fn ld_i(&mut self, instruction: u16) -> usize {
        let addr = instruction & 0xFFF;

        self.i = addr;
        self.program_counter + 2
    }

    /// Draws n-byte sprite from Vi at Vx, Vy (0xDxyn)
    fn drw(&mut self, instruction: u16, display: &mut Display) -> usize {
        let vx = instruction >> 8 & 0xF;
        let vy = instruction >> 4 & 0xF;
        let n = (instruction & 0xF) as usize;

        let x = self.registers[vx as usize] as usize;
        let y = self.registers[vy as usize] as usize;
        let mut vf = false;

        for j in 0..n {
            let byte = self.memory[self.i as usize + j].reverse_bits();

            for i in 0..8 {
                let x = (x + i) % 64;
                let y = (y + j) % 32;
                let bit = (byte >> i) % 2 == 1;

                let before = display.pixels[y][x];
                display.pixels[y][x] ^= bit;
                let after = display.pixels[y][x];

                if before && !after {
                    vf = true;
                }
            }
        }

        self.registers[0xF] = vf as u8;
        self.program_counter + 2
    }

    /// Adds Vx to Vi (0xFx1E)
    fn add_i_v(&mut self, instruction: u16) -> usize {
        let vx = instruction >> 8 & 0xF;

        self.i = self.i.wrapping_add(self.registers[vx as usize] as u16);
        self.program_counter + 2
    }

    /// Loads [V0, Vx] to memory starting at Vi (0xFx55)
    fn ld_i_v(&mut self, instruction: u16) -> usize {
        let vx = instruction >> 8 & 0xF;

        for index in 0..=vx {
            self.memory[(self.i + index) as usize] = self.registers[index as usize]
        }

        self.program_counter + 2
    }

    /// Loads memory starting at Vi to [V0, Vx] (0xFx65)
    fn ld_v_i(&mut self, instruction: u16) -> usize {
        let vx = instruction >> 8 & 0xF;

        for index in 0..=vx {
            self.registers[index as usize] = self.memory[(self.i + index) as usize]
        }

        self.program_counter + 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cls() {
        let mut emulator = Emulator::new(&[0x00, 0xE0]);
        let mut display = Display::default();
        display.pixels[0][0] = true;
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard);

        assert_eq!(emulator.program_counter, 0x202);
        assert_eq!(display.pixels[0][0], false);
    }

    #[test]
    fn test_ret() {
        let mut emulator = Emulator::new(&[0x00, 0xEE]);
        emulator.stack.push(0x400);
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard);

        assert_eq!(emulator.program_counter, 0x402);
        assert_eq!(emulator.stack, vec![]);
    }

    #[test]
    fn test_jp() {
        let mut emulator = Emulator::new(&[0x12, 0x34]);
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard);

        assert_eq!(emulator.program_counter, 0x234);
        assert_eq!(emulator.stack, vec![]);
    }

    #[test]
    fn test_call() {
        let mut emulator = Emulator::new(&[0x23, 0x45]);
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard);

        assert_eq!(emulator.program_counter, 0x345);
        assert_eq!(emulator.stack, vec![0x200]);
    }

    #[test]
    fn test_se_v_equal() {
        let mut emulator = Emulator::new(&[0x34, 0x56]);
        emulator.registers[0x4] = 0x56;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard);

        assert_eq!(emulator.program_counter, 0x204);
    }

    #[test]
    fn test_se_v_not_equal() {
        let mut emulator = Emulator::new(&[0x34, 0x56]);
        emulator.registers[0x4] = 0x65;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard);

        assert_eq!(emulator.program_counter, 0x202);
    }

    #[test]
    fn test_sne_v_equal() {
        let mut emulator = Emulator::new(&[0x45, 0x67]);
        emulator.registers[0x5] = 0x67;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard);

        assert_eq!(emulator.program_counter, 0x202);
    }

    #[test]
    fn test_sne_v_not_equal() {
        let mut emulator = Emulator::new(&[0x45, 0x67]);
        emulator.registers[0x5] = 0x76;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard);

        assert_eq!(emulator.program_counter, 0x204);
    }

    #[test]
    fn test_ld_v() {
        let mut emulator = Emulator::new(&[0x67, 0x89]);
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard);

        assert_eq!(emulator.program_counter, 0x202);
        assert_eq!(emulator.registers[0x7], 0x89);
    }

    #[test]
    fn test_add_v() {
        let mut emulator = Emulator::new(&[0x78, 0x9A, 0x78, 0x9A]);
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard);

        assert_eq!(emulator.program_counter, 0x202);
        assert_eq!(emulator.registers[0x8], 0x9A);

        emulator.tick(&mut display, &keyboard);

        assert_eq!(emulator.program_counter, 0x204);
        assert_eq!(emulator.registers[0x8], 0x34);
    }

    #[test]
    fn test_ld_v_v() {
        let mut emulator = Emulator::new(&[0x89, 0xA0]);
        emulator.registers[0x9] = 0x20;
        emulator.registers[0xA] = 0x40;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard);

        assert_eq!(emulator.program_counter, 0x202);
        assert_eq!(emulator.registers[0x9], 0x40);
    }

    #[test]
    fn test_or_v_v() {
        let mut emulator = Emulator::new(&[0x89, 0xA1]);
        emulator.registers[0x9] = 0b11110000;
        emulator.registers[0xA] = 0b11001100;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard);

        assert_eq!(emulator.program_counter, 0x202);
        assert_eq!(emulator.registers[0x9], 0b11111100);
    }

    #[test]
    fn test_and_v_v() {
        let mut emulator = Emulator::new(&[0x89, 0xA2]);
        emulator.registers[0x9] = 0b11110000;
        emulator.registers[0xA] = 0b11001100;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard);

        assert_eq!(emulator.program_counter, 0x202);
        assert_eq!(emulator.registers[0x9], 0b11000000);
    }

    #[test]
    fn test_xor_v_v() {
        let mut emulator = Emulator::new(&[0x89, 0xA3]);
        emulator.registers[0x9] = 0b11110000;
        emulator.registers[0xA] = 0b11001100;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard);

        assert_eq!(emulator.program_counter, 0x202);
        assert_eq!(emulator.registers[0x9], 0b00111100);
    }

    #[test]
    fn test_add_v_v() {
        let mut emulator = Emulator::new(&[0x89, 0xA4, 0x89, 0xA4]);
        emulator.registers[0x9] = 0x78;
        emulator.registers[0xA] = 0x78;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard);

        assert_eq!(emulator.program_counter, 0x202);
        assert_eq!(emulator.registers[0x9], 0xF0);
        assert_eq!(emulator.registers[0xF], 0);

        emulator.tick(&mut display, &keyboard);

        assert_eq!(emulator.program_counter, 0x204);
        assert_eq!(emulator.registers[0x9], 0x68);
        assert_eq!(emulator.registers[0xF], 1);
    }

    #[test]
    fn test_shr_v_v() {
        let mut emulator = Emulator::new(&[0x89, 0xA6, 0x89, 0xA6]);
        emulator.registers[0x9] = 0b00000101;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard);

        assert_eq!(emulator.program_counter, 0x202);
        assert_eq!(emulator.registers[0x9], 0b00000010);
        assert_eq!(emulator.registers[0xF], 1);

        emulator.tick(&mut display, &keyboard);

        assert_eq!(emulator.program_counter, 0x204);
        assert_eq!(emulator.registers[0x9], 0b00000001);
        assert_eq!(emulator.registers[0xF], 0);
    }

    #[test]
    fn test_shl_v_v() {
        let mut emulator = Emulator::new(&[0x89, 0xAE, 0x89, 0xAE]);
        emulator.registers[0x9] = 0b10100000;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard);

        assert_eq!(emulator.program_counter, 0x202);
        assert_eq!(emulator.registers[0x9], 0b01000000);
        assert_eq!(emulator.registers[0xF], 1);

        emulator.tick(&mut display, &keyboard);

        assert_eq!(emulator.program_counter, 0x204);
        assert_eq!(emulator.registers[0x9], 0b10000000);
        assert_eq!(emulator.registers[0xF], 0);
    }

    #[test]
    fn test_ld_i() {
        let mut emulator = Emulator::new(&[0xAB, 0xCD]);
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard);

        assert_eq!(emulator.i, 0xBCD);
        assert_eq!(emulator.program_counter, 0x202);
    }

    #[test]
    fn test_drw() {
        let mut emulator = Emulator::new(&[0xDA, 0xB2]);
        emulator.i = 0x400;
        emulator.memory[0x400] = 0b11110000;
        emulator.memory[0x401] = 0b11001100;
        emulator.registers[0xA] = 0x3E;
        emulator.registers[0xB] = 0x2;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard);

        assert_eq!(display.pixels[0x2][0x3E], true);
        assert_eq!(display.pixels[0x2][0x3F], true);
        assert_eq!(display.pixels[0x2][0x0], true);
        assert_eq!(display.pixels[0x2][0x1], true);
        assert_eq!(display.pixels[0x2][0x2], false);
        assert_eq!(display.pixels[0x2][0x3], false);
        assert_eq!(display.pixels[0x2][0x4], false);
        assert_eq!(display.pixels[0x2][0x5], false);

        assert_eq!(display.pixels[0x3][0x3E], true);
        assert_eq!(display.pixels[0x3][0x3F], true);
        assert_eq!(display.pixels[0x3][0x0], false);
        assert_eq!(display.pixels[0x3][0x1], false);
        assert_eq!(display.pixels[0x3][0x2], true);
        assert_eq!(display.pixels[0x3][0x3], true);
        assert_eq!(display.pixels[0x3][0x4], false);
        assert_eq!(display.pixels[0x3][0x5], false);

        assert_eq!(emulator.program_counter, 0x202);
    }

    #[test]
    fn test_add_i_v() {
        let mut emulator = Emulator::new(&[0xF5, 0x1E]);
        emulator.i = 0x9A;
        emulator.registers[0x5] = 0x9A;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard);

        assert_eq!(emulator.i, 0x134);
        assert_eq!(emulator.program_counter, 0x202);
        assert_eq!(emulator.registers[0xF], 0);
    }

    #[test]
    fn test_ld_i_v() {
        let mut emulator = Emulator::new(&[0xF8, 0x55]);
        emulator.i = 0x400;
        emulator.registers[0x0] = 0x1;
        emulator.registers[0x4] = 0x5;
        emulator.registers[0x8] = 0x9;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard);

        assert_eq!(emulator.memory[0x400], 0x1);
        assert_eq!(emulator.memory[0x404], 0x5);
        assert_eq!(emulator.memory[0x408], 0x9);
        assert_eq!(emulator.program_counter, 0x202);
    }

    #[test]
    fn test_ld_v_i() {
        let mut emulator = Emulator::new(&[0xF8, 0x65]);
        emulator.i = 0x400;
        emulator.memory[0x400] = 0x1;
        emulator.memory[0x404] = 0x5;
        emulator.memory[0x408] = 0x9;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard);

        assert_eq!(emulator.registers[0x0], 0x1);
        assert_eq!(emulator.registers[0x4], 0x5);
        assert_eq!(emulator.registers[0x8], 0x9);
        assert_eq!(emulator.program_counter, 0x202);
    }
}
