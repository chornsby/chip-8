use crate::display::Display;
use crate::instruction::Instruction;
use crate::keyboard::Keyboard;
use crate::memory::{Memory, PROGRAM_OFFSET};
use rand::Rng;
use std::convert::TryFrom;

pub struct Emulator {
    memory: Memory,
    registers: [u8; 16],
    i: u16,
    delay_timer: u8,
    sound_timer: u8,
    program_counter: usize,
    stack: Vec<usize>,
}

impl Emulator {
    /// Loads a rom into memory and initialise the emulator
    pub fn new(rom: &[u8]) -> Self {
        let memory = Memory::new(rom);

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

    /// Ticks down 1/60th of a second on the delay and sound timers
    pub fn decrement_timers(&mut self) {
        self.delay_timer = self.delay_timer.saturating_sub(1);
        self.sound_timer = self.sound_timer.saturating_sub(1);
    }

    /// Returns true if the Chip-8 buzzer is active
    pub fn is_sound_playing(&self) -> bool {
        0 < self.sound_timer
    }

    /// Evaluates one CPU instruction and updates the program counter
    pub fn tick(&mut self, display: &mut Display, keyboard: &Keyboard) -> Result<(), String> {
        let bytes = self.memory.get_instruction(self.program_counter);
        let instruction = Instruction::try_from(bytes)?;

        self.program_counter = match instruction {
            Instruction::Cls => self.cls(display),
            Instruction::Ret => self.ret(),
            Instruction::Jp { addr } => self.jp(addr),
            Instruction::Call { addr } => self.call(addr),
            Instruction::SeV { vx, byte } => self.se_v(vx, byte),
            Instruction::SneV { vx, byte } => self.sne_v(vx, byte),
            Instruction::SeVV { vx, vy } => self.se_v_v(vx, vy),
            Instruction::LdV { vx, byte } => self.ld_v(vx, byte),
            Instruction::AddV { vx, byte } => self.add_v(vx, byte),
            Instruction::LdVV { vx, vy } => self.ld_v_v(vx, vy),
            Instruction::OrVV { vx, vy } => self.or_v_v(vx, vy),
            Instruction::AndVV { vx, vy } => self.and_v_v(vx, vy),
            Instruction::XorVV { vx, vy } => self.xor_v_v(vx, vy),
            Instruction::AddVV { vx, vy } => self.add_v_v(vx, vy),
            Instruction::SubVV { vx, vy } => self.sub_v_v(vx, vy),
            Instruction::ShrVV { vx, .. } => self.shr_v_v(vx),
            Instruction::SubnVV { vx, vy } => self.subn_v_v(vx, vy),
            Instruction::ShlVV { vx, .. } => self.shl_v_v(vx),
            Instruction::SneVV { vx, vy } => self.sne_v_v(vx, vy),
            Instruction::LdI { addr } => self.ld_i(addr),
            Instruction::JpV { addr } => self.jp_v(addr),
            Instruction::RndV { vx, byte } => self.rnd_v(vx, byte),
            Instruction::Drw { vx, vy, n } => self.drw(vx, vy, n, display),
            Instruction::SkpV { vx } => self.skp_v(vx, keyboard),
            Instruction::SknpV { vx } => self.sknp_v(vx, keyboard),
            Instruction::LdVDt { vx } => self.ld_v_dt(vx),
            Instruction::LdVK { vx } => self.ld_v_k(vx, keyboard),
            Instruction::LdDtV { vx } => self.ld_dt_v(vx),
            Instruction::LdStV { vx } => self.ld_st_v(vx),
            Instruction::AddIV { vx } => self.add_i_v(vx),
            Instruction::LdFV { vx } => self.ld_f_v(vx),
            Instruction::LdBV { vx } => self.ld_b_v(vx),
            Instruction::LdIV { vx } => self.ld_i_v(vx),
            Instruction::LdVI { vx } => self.ld_v_i(vx),
        };

        Ok(())
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
    fn jp(&self, addr: usize) -> usize {
        addr
    }

    /// Calls subroutine at nnn (0x2nnn)
    fn call(&mut self, addr: usize) -> usize {
        self.stack.push(self.program_counter);
        addr
    }

    /// Skips an instruction if Vx == kk (0x3xkk)
    fn se_v(&self, vx: usize, byte: u8) -> usize {
        if self.registers[vx] == byte {
            self.program_counter + 4
        } else {
            self.program_counter + 2
        }
    }

    /// Skips an instruction if Vx != kk (0x4xkk)
    fn sne_v(&self, vx: usize, byte: u8) -> usize {
        if self.registers[vx] == byte {
            self.program_counter + 2
        } else {
            self.program_counter + 4
        }
    }

    /// Skips an instruction if Vx == Vy (0x5xy0)
    fn se_v_v(&self, vx: usize, vy: usize) -> usize {
        if self.registers[vx] == self.registers[vy] {
            self.program_counter + 4
        } else {
            self.program_counter + 2
        }
    }

    /// Loads kk to Vx (0x6xkk)
    fn ld_v(&mut self, vx: usize, byte: u8) -> usize {
        self.registers[vx] = byte;
        self.program_counter + 2
    }

    /// Adds kk to Vx (0x7xkk)
    fn add_v(&mut self, vx: usize, byte: u8) -> usize {
        self.registers[vx] = self.registers[vx].wrapping_add(byte);
        self.program_counter + 2
    }

    /// Sets Vy to Vx (0x8xy0)
    fn ld_v_v(&mut self, vx: usize, vy: usize) -> usize {
        self.registers[vx] = self.registers[vy];
        self.program_counter + 2
    }

    /// Stores bitwise OR of Vx and Vy to Vx (0x8xy1)
    fn or_v_v(&mut self, vx: usize, vy: usize) -> usize {
        self.registers[vx] |= self.registers[vy];
        self.program_counter + 2
    }

    /// Stores bitwise AND of Vx and Vy in Vx (0x8xy2)
    fn and_v_v(&mut self, vx: usize, vy: usize) -> usize {
        self.registers[vx] &= self.registers[vy];
        self.program_counter + 2
    }

    /// Stores bitwise XOR of Vx and Vy in Vx (0x8xy3)
    fn xor_v_v(&mut self, vx: usize, vy: usize) -> usize {
        self.registers[vx] ^= self.registers[vy];
        self.program_counter + 2
    }

    /// Adds Vx and Vy with carry (0x8xy4)
    fn add_v_v(&mut self, vx: usize, vy: usize) -> usize {
        let (value, carry) = self.registers[vx].overflowing_add(self.registers[vy]);

        self.registers[0xF] = carry as u8;
        self.registers[vx] = value;
        self.program_counter + 2
    }

    /// Subtracts Vy from Vx with carry (0x8xy5)
    fn sub_v_v(&mut self, vx: usize, vy: usize) -> usize {
        let (value, carry) = self.registers[vx].overflowing_sub(self.registers[vy]);

        self.registers[0xF] = !carry as u8;
        self.registers[vx] = value;
        self.program_counter + 2
    }

    /// Shifts Vx to the right with carry (0x8xy6)
    fn shr_v_v(&mut self, vx: usize) -> usize {
        self.registers[0xF] = self.registers[vx] % 2;
        self.registers[vx] >>= 1;
        self.program_counter + 2
    }

    /// Subtracts Vy from Vx and stores result in Vx with carry (0x8xy7)
    fn subn_v_v(&mut self, vx: usize, vy: usize) -> usize {
        let (value, carry) = self.registers[vy].overflowing_sub(self.registers[vx]);

        self.registers[0xF] = !carry as u8;
        self.registers[vx] = value;
        self.program_counter + 2
    }

    /// Shifts Vx to the left with carry (0x8xyE)
    fn shl_v_v(&mut self, vx: usize) -> usize {
        self.registers[0xF] = (0b10000000 <= self.registers[vx]) as u8;
        self.registers[vx] <<= 1;
        self.program_counter + 2
    }

    /// Skips the next instruction if Vx != Vy (0x9xy0)
    fn sne_v_v(&self, vx: usize, vy: usize) -> usize {
        if self.registers[vx] == self.registers[vy] {
            self.program_counter + 2
        } else {
            self.program_counter + 4
        }
    }

    /// Loads nnn to Vi (0xAnnn)
    fn ld_i(&mut self, addr: usize) -> usize {
        self.i = addr as u16;
        self.program_counter + 2
    }

    /// Jumps program counter to nnn + V0 (0xBnnn)
    fn jp_v(&self, addr: usize) -> usize {
        addr + self.registers[0x0] as usize
    }

    /// Randomly generates a random number to store in Vx (0xCxkk)
    fn rnd_v(&mut self, vx: usize, byte: u8) -> usize {
        self.registers[vx] = rand::thread_rng().gen::<u8>() & byte;
        self.program_counter + 2
    }

    /// Draws n-byte sprite from Vi at Vx, Vy (0xDxyn)
    fn drw(&mut self, vx: usize, vy: usize, n: usize, display: &mut Display) -> usize {
        let offset = self.i as usize;
        let x = self.registers[vx] as usize;
        let y = self.registers[vy] as usize;

        let sprite = self.memory.get_sprite(offset, n);
        let erased = display.xor_sprite(x, y, sprite);

        self.registers[0xF] = erased as u8;
        self.program_counter + 2
    }

    /// Skips the next instruction if Vx is pressed (0xEx9E)
    fn skp_v(&self, vx: usize, keyboard: &Keyboard) -> usize {
        if keyboard.is_pressed(&self.registers[vx].into()) {
            self.program_counter + 4
        } else {
            self.program_counter + 2
        }
    }

    /// Skips the next instruction if Vx is not pressed (0xExA1)
    fn sknp_v(&self, vx: usize, keyboard: &Keyboard) -> usize {
        if keyboard.is_pressed(&self.registers[vx].into()) {
            self.program_counter + 2
        } else {
            self.program_counter + 4
        }
    }

    /// Loads the delay timer into Vx (0xFx07)
    fn ld_v_dt(&mut self, vx: usize) -> usize {
        self.registers[vx] = self.delay_timer;
        self.program_counter + 2
    }

    /// Stops execution until a key press then stores it in Vx (0xFx0A)
    fn ld_v_k(&mut self, vx: usize, keyboard: &Keyboard) -> usize {
        let key_pressed = Keyboard::keys().find(|key| keyboard.is_pressed(key));

        if let Some(key) = key_pressed {
            self.registers[vx] = key as u8;
            self.program_counter + 2
        } else {
            self.program_counter
        }
    }

    /// Loads Vx into the delay timer (0xFx15)
    fn ld_dt_v(&mut self, vx: usize) -> usize {
        self.delay_timer = self.registers[vx];
        self.program_counter + 2
    }

    /// Loads Vx into the sound timer (0xFx18)
    fn ld_st_v(&mut self, vx: usize) -> usize {
        self.sound_timer = self.registers[vx];
        self.program_counter + 2
    }

    /// Adds Vx to Vi (0xFx1E)
    fn add_i_v(&mut self, vx: usize) -> usize {
        self.i = self.i.wrapping_add(self.registers[vx] as u16);
        self.program_counter + 2
    }

    /// Sets Vi to the location of sprite Vx (0xFx29)
    fn ld_f_v(&mut self, vx: usize) -> usize {
        let value = self.registers[vx];

        self.i = Memory::calculate_digit_offset(value) as u16;
        self.program_counter + 2
    }

    /// Store BCD representation of Vx in memory (0xFx33)
    fn ld_b_v(&mut self, vx: usize) -> usize {
        let x = self.registers[vx];

        for index in 0..3 {
            let offset = (self.i + index) as usize;
            let digit = (x / 10u8.pow(2 - index as u32)) % 10;
            self.memory.set_byte(offset, digit);
        }

        self.program_counter + 2
    }

    /// Loads [V0, Vx] to memory starting at Vi (0xFx55)
    fn ld_i_v(&mut self, vx: usize) -> usize {
        for index in 0..=vx {
            self.memory
                .set_byte(self.i as usize + index, self.registers[index])
        }

        self.program_counter + 2
    }

    /// Loads memory starting at Vi to [V0, Vx] (0xFx65)
    fn ld_v_i(&mut self, vx: usize) -> usize {
        for index in 0..=vx {
            self.registers[index] = self.memory.get_byte(self.i as usize + index);
        }

        self.program_counter + 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keyboard::Key;

    #[test]
    fn test_cls() {
        let mut emulator = Emulator::new(&[0x00, 0xE0]);
        let mut display = Display::new(&[(0, 0)]);
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x202);
        assert_eq!(display.get_pixel(0, 0), false);
    }

    #[test]
    fn test_ret() {
        let mut emulator = Emulator::new(&[0x00, 0xEE]);
        emulator.stack.push(0x400);
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x402);
        assert_eq!(emulator.stack, vec![]);
    }

    #[test]
    fn test_jp() {
        let mut emulator = Emulator::new(&[0x12, 0x34]);
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x234);
        assert_eq!(emulator.stack, vec![]);
    }

    #[test]
    fn test_call() {
        let mut emulator = Emulator::new(&[0x23, 0x45]);
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x345);
        assert_eq!(emulator.stack, vec![0x200]);
    }

    #[test]
    fn test_se_v_equal() {
        let mut emulator = Emulator::new(&[0x34, 0x56]);
        emulator.registers[0x4] = 0x56;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x204);
    }

    #[test]
    fn test_se_v_not_equal() {
        let mut emulator = Emulator::new(&[0x34, 0x56]);
        emulator.registers[0x4] = 0x65;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x202);
    }

    #[test]
    fn test_sne_v_equal() {
        let mut emulator = Emulator::new(&[0x45, 0x67]);
        emulator.registers[0x5] = 0x67;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x202);
    }

    #[test]
    fn test_sne_v_not_equal() {
        let mut emulator = Emulator::new(&[0x45, 0x67]);
        emulator.registers[0x5] = 0x76;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x204);
    }

    #[test]
    fn test_se_v_v_equal() {
        let mut emulator = Emulator::new(&[0x56, 0x70]);
        emulator.registers[0x6] = 0x78;
        emulator.registers[0x7] = 0x78;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x204);
    }

    #[test]
    fn test_se_v_v_not_equal() {
        let mut emulator = Emulator::new(&[0x56, 0x70]);
        emulator.registers[0x6] = 0x78;
        emulator.registers[0x7] = 0x89;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x202);
    }

    #[test]
    fn test_ld_v() {
        let mut emulator = Emulator::new(&[0x67, 0x89]);
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x202);
        assert_eq!(emulator.registers[0x7], 0x89);
    }

    #[test]
    fn test_add_v() {
        let mut emulator = Emulator::new(&[0x78, 0x9A, 0x78, 0x9A]);
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x202);
        assert_eq!(emulator.registers[0x8], 0x9A);

        emulator.tick(&mut display, &keyboard).unwrap();

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

        emulator.tick(&mut display, &keyboard).unwrap();

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

        emulator.tick(&mut display, &keyboard).unwrap();

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

        emulator.tick(&mut display, &keyboard).unwrap();

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

        emulator.tick(&mut display, &keyboard).unwrap();

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

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x202);
        assert_eq!(emulator.registers[0x9], 0xF0);
        assert_eq!(emulator.registers[0xF], 0);

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x204);
        assert_eq!(emulator.registers[0x9], 0x68);
        assert_eq!(emulator.registers[0xF], 1);
    }

    #[test]
    fn test_sub_v_v() {
        let mut emulator = Emulator::new(&[0x89, 0xA5, 0x89, 0xA5]);
        emulator.registers[0x9] = 0x78;
        emulator.registers[0xA] = 0x78;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x202);
        assert_eq!(emulator.registers[0x9], 0x0);
        assert_eq!(emulator.registers[0xF], 1);

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x204);
        assert_eq!(emulator.registers[0x9], 0x88);
        assert_eq!(emulator.registers[0xF], 0);
    }

    #[test]
    fn test_shr_v_v() {
        let mut emulator = Emulator::new(&[0x89, 0xA6, 0x89, 0xA6]);
        emulator.registers[0x9] = 0b00000101;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x202);
        assert_eq!(emulator.registers[0x9], 0b00000010);
        assert_eq!(emulator.registers[0xF], 1);

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x204);
        assert_eq!(emulator.registers[0x9], 0b00000001);
        assert_eq!(emulator.registers[0xF], 0);
    }

    #[test]
    fn test_subn_v_v() {
        let mut emulator = Emulator::new(&[0x89, 0xA7, 0x89, 0xA7]);
        emulator.registers[0x9] = 0x78;
        emulator.registers[0xA] = 0x78;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x202);
        assert_eq!(emulator.registers[0x9], 0x0);
        assert_eq!(emulator.registers[0xF], 1);

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x204);
        assert_eq!(emulator.registers[0x9], 0x78);
        assert_eq!(emulator.registers[0xF], 1);
    }

    #[test]
    fn test_shl_v_v() {
        let mut emulator = Emulator::new(&[0x89, 0xAE, 0x89, 0xAE]);
        emulator.registers[0x9] = 0b10100000;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x202);
        assert_eq!(emulator.registers[0x9], 0b01000000);
        assert_eq!(emulator.registers[0xF], 1);

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x204);
        assert_eq!(emulator.registers[0x9], 0b10000000);
        assert_eq!(emulator.registers[0xF], 0);
    }

    #[test]
    fn test_sne_v_v_not_equal() {
        let mut emulator = Emulator::new(&[0x9A, 0xB0]);
        emulator.registers[0xA] = 0xC;
        emulator.registers[0xB] = 0xD;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x204);
    }

    #[test]
    fn test_sne_v_v_equal() {
        let mut emulator = Emulator::new(&[0x9A, 0xB0]);
        emulator.registers[0xA] = 0xC;
        emulator.registers[0xB] = 0xC;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x202);
    }

    #[test]
    fn test_ld_i() {
        let mut emulator = Emulator::new(&[0xAB, 0xCD]);
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.i, 0xBCD);
        assert_eq!(emulator.program_counter, 0x202);
    }

    #[test]
    fn test_jp_v() {
        let mut emulator = Emulator::new(&[0xBC, 0xDE]);
        emulator.registers[0x0] = 0x1;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0xCDF);
        assert_eq!(emulator.stack, vec![]);
    }

    #[test]
    fn test_drw() {
        let mut emulator = Emulator::new(&[0xDA, 0xB2]);
        emulator.i = 0x400;
        emulator.memory.set_byte(0x400, 0b11110000);
        emulator.memory.set_byte(0x401, 0b11001100);
        emulator.registers[0xA] = 0x3E;
        emulator.registers[0xB] = 0x2;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(display.get_pixel(0x3E, 0x2), true);
        assert_eq!(display.get_pixel(0x3F, 0x2), true);
        assert_eq!(display.get_pixel(0x0, 0x2), true);
        assert_eq!(display.get_pixel(0x1, 0x2), true);
        assert_eq!(display.get_pixel(0x2, 0x2), false);
        assert_eq!(display.get_pixel(0x3, 0x2), false);
        assert_eq!(display.get_pixel(0x4, 0x2), false);
        assert_eq!(display.get_pixel(0x5, 0x2), false);

        assert_eq!(display.get_pixel(0x3E, 0x3), true);
        assert_eq!(display.get_pixel(0x3F, 0x3), true);
        assert_eq!(display.get_pixel(0x0, 0x3), false);
        assert_eq!(display.get_pixel(0x1, 0x3), false);
        assert_eq!(display.get_pixel(0x2, 0x3), true);
        assert_eq!(display.get_pixel(0x3, 0x3), true);
        assert_eq!(display.get_pixel(0x4, 0x3), false);
        assert_eq!(display.get_pixel(0x5, 0x3), false);

        assert_eq!(emulator.program_counter, 0x202);
    }

    #[test]
    fn test_skp_v_pressed() {
        let mut emulator = Emulator::new(&[0xE0, 0x9E]);
        emulator.registers[0x0] = 0x5;
        let mut display = Display::default();
        let mut keyboard = Keyboard::default();
        keyboard.press(&Key::Num5);

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x204);
    }

    #[test]
    fn test_skp_v_not_pressed() {
        let mut emulator = Emulator::new(&[0xE0, 0x9E]);
        emulator.registers[0x0] = 0x5;
        let mut display = Display::default();
        let mut keyboard = Keyboard::default();
        keyboard.release(&Key::Num5);

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x202);
    }

    #[test]
    fn test_sknp_v_pressed() {
        let mut emulator = Emulator::new(&[0xE0, 0xA1]);
        emulator.registers[0x0] = 0x5;
        let mut display = Display::default();
        let mut keyboard = Keyboard::default();
        keyboard.press(&Key::Num5);

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x202);
    }

    #[test]
    fn test_sknp_v_not_pressed() {
        let mut emulator = Emulator::new(&[0xE0, 0xA1]);
        emulator.registers[0x0] = 0x5;
        let mut display = Display::default();
        let mut keyboard = Keyboard::default();
        keyboard.release(&Key::Num5);

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x204);
    }

    #[test]
    fn test_ld_v_dt() {
        let mut emulator = Emulator::new(&[0xF1, 0x07]);
        emulator.delay_timer = 0x55;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x202);
        assert_eq!(emulator.registers[0x1], 0x55);
    }

    #[test]
    fn test_ld_v_k() {
        let mut emulator = Emulator::new(&[0xF2, 0x0A]);
        let mut display = Display::default();
        let mut keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x200);
        assert_eq!(emulator.registers[0x2], 0x0);

        keyboard.press(&Key::Num2);

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x202);
        assert_eq!(emulator.registers[0x2], 0x2);
    }

    #[test]
    fn test_ld_dt_v() {
        let mut emulator = Emulator::new(&[0xF3, 0x15]);
        emulator.registers[0x3] = 3;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x202);
        assert_eq!(emulator.delay_timer, 0x3);
    }

    #[test]
    fn test_ld_st_v() {
        let mut emulator = Emulator::new(&[0xF4, 0x18]);
        emulator.registers[0x4] = 4;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.program_counter, 0x202);
        assert_eq!(emulator.sound_timer, 0x4);
    }

    #[test]
    fn test_add_i_v() {
        let mut emulator = Emulator::new(&[0xF5, 0x1E]);
        emulator.i = 0x9A;
        emulator.registers[0x5] = 0x9A;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.i, 0x134);
        assert_eq!(emulator.program_counter, 0x202);
        assert_eq!(emulator.registers[0xF], 0);
    }

    #[test]
    fn test_ld_f_v() {
        let mut emulator = Emulator::new(&[0xF6, 0x29]);
        emulator.registers[0x6] = 0xA;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.i, 0xA * 5);
        assert_eq!(emulator.program_counter, 0x202);
    }

    #[test]
    fn test_ld_b_v() {
        let mut emulator = Emulator::new(&[0xF7, 0x33]);
        emulator.i = 0x400;
        emulator.registers[0x7] = 0x7B;
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.memory.get_byte(0x400), 0x1);
        assert_eq!(emulator.memory.get_byte(0x401), 0x2);
        assert_eq!(emulator.memory.get_byte(0x402), 0x3);
        assert_eq!(emulator.program_counter, 0x202);
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

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.memory.get_byte(0x400), 0x1);
        assert_eq!(emulator.memory.get_byte(0x404), 0x5);
        assert_eq!(emulator.memory.get_byte(0x408), 0x9);
        assert_eq!(emulator.program_counter, 0x202);
    }

    #[test]
    fn test_ld_v_i() {
        let mut emulator = Emulator::new(&[0xF8, 0x65]);
        emulator.i = 0x400;
        emulator.memory.set_byte(0x400, 0x1);
        emulator.memory.set_byte(0x404, 0x5);
        emulator.memory.set_byte(0x408, 0x9);
        let mut display = Display::default();
        let keyboard = Keyboard::default();

        emulator.tick(&mut display, &keyboard).unwrap();

        assert_eq!(emulator.registers[0x0], 0x1);
        assert_eq!(emulator.registers[0x4], 0x5);
        assert_eq!(emulator.registers[0x8], 0x9);
        assert_eq!(emulator.program_counter, 0x202);
    }
}
