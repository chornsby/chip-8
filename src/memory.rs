use crate::rom::Rom;

const MEMORY_SIZE: usize = 0x1000;
pub const PROGRAM_OFFSET: usize = 0x200;

pub struct Memory(pub [u8; MEMORY_SIZE]);

impl Default for Memory {
    fn default() -> Self {
        Self([0; MEMORY_SIZE])
    }
}

impl Memory {
    pub fn load_rom(&mut self, rom: &Rom) {
        for (index, &byte) in rom.0.iter().enumerate() {
            self.0[index + PROGRAM_OFFSET] = byte;
        }
    }
}
