use std::convert::TryFrom;

/// A parsed instruction for the Chip-8 CPU
pub enum Instruction {
    Cls,
    Ret,
    Jp { addr: usize },
    Call { addr: usize },
    SeV { vx: usize, byte: u8 },
    SneV { vx: usize, byte: u8 },
    SeVV { vx: usize, vy: usize },
    LdV { vx: usize, byte: u8 },
    AddV { vx: usize, byte: u8 },
    LdVV { vx: usize, vy: usize },
    OrVV { vx: usize, vy: usize },
    AndVV { vx: usize, vy: usize },
    XorVV { vx: usize, vy: usize },
    AddVV { vx: usize, vy: usize },
    SubVV { vx: usize, vy: usize },
    ShrVV { vx: usize, vy: usize },
    SubnVV { vx: usize, vy: usize },
    ShlVV { vx: usize, vy: usize },
    SneVV { vx: usize, vy: usize },
    LdI { addr: usize },
    JpV { addr: usize },
    RndV { vx: usize, byte: u8 },
    Drw { vx: usize, vy: usize, n: usize },
    SkpV { vx: usize },
    SknpV { vx: usize },
    LdVDt { vx: usize },
    LdVK { vx: usize },
    LdDtV { vx: usize },
    LdStV { vx: usize },
    AddIV { vx: usize },
    LdFV { vx: usize },
    LdBV { vx: usize },
    LdIV { vx: usize },
    LdVI { vx: usize },
}

impl TryFrom<u16> for Instruction {
    type Error = String;

    /// Try to parse the two-byte instruction
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let addr = (value & 0xFFF) as usize;
        let byte = (value & 0xFF) as u8;
        let vx = (value >> 8 & 0xF) as usize;
        let vy = (value >> 4 & 0xF) as usize;
        let n = (value & 0xF) as usize;

        match value {
            0x00E0 => Ok(Self::Cls),
            0x00EE => Ok(Self::Ret),
            0x1000..=0x1FFF => Ok(Self::Jp { addr }),
            0x2000..=0x2FFF => Ok(Self::Call { addr }),
            0x3000..=0x3FFF => Ok(Self::SeV { vx, byte }),
            0x4000..=0x4FFF => Ok(Self::SneV { vx, byte }),
            0x5000..=0x5FFF if value & 0xF == 0x0 => Ok(Self::SeVV { vx, vy }),
            0x6000..=0x6FFF => Ok(Self::LdV { vx, byte }),
            0x7000..=0x7FFF => Ok(Self::AddV { vx, byte }),
            0x8000..=0x8FFF => match value & 0xF {
                0x0 => Ok(Self::LdVV { vx, vy }),
                0x1 => Ok(Self::OrVV { vx, vy }),
                0x2 => Ok(Self::AndVV { vx, vy }),
                0x3 => Ok(Self::XorVV { vx, vy }),
                0x4 => Ok(Self::AddVV { vx, vy }),
                0x5 => Ok(Self::SubVV { vx, vy }),
                0x6 => Ok(Self::ShrVV { vx, vy }),
                0x7 => Ok(Self::SubnVV { vx, vy }),
                0xE => Ok(Self::ShlVV { vx, vy }),
                _ => Err(format!("Unknown instruction 0x{:X}", value)),
            },
            0x9000..=0x9FFF if value & 0xF == 0x0 => Ok(Self::SneVV { vx, vy }),
            0xA000..=0xAFFF => Ok(Self::LdI { addr }),
            0xB000..=0xBFFF => Ok(Self::JpV { addr }),
            0xC000..=0xCFFF => Ok(Self::RndV { vx, byte }),
            0xD000..=0xDFFF => Ok(Self::Drw { vx, vy, n }),
            0xE000..=0xEFFF => match value & 0xFF {
                0x9E => Ok(Self::SkpV { vx }),
                0xA1 => Ok(Self::SknpV { vx }),
                _ => Err(format!("Unknown instruction 0x{:X}", value)),
            },
            0xF000..=0xFFFF => match value & 0xFF {
                0x07 => Ok(Self::LdVDt { vx }),
                0x0A => Ok(Self::LdVK { vx }),
                0x15 => Ok(Self::LdDtV { vx }),
                0x18 => Ok(Self::LdStV { vx }),
                0x1E => Ok(Self::AddIV { vx }),
                0x29 => Ok(Self::LdFV { vx }),
                0x33 => Ok(Self::LdBV { vx }),
                0x55 => Ok(Self::LdIV { vx }),
                0x65 => Ok(Self::LdVI { vx }),
                _ => Err(format!("Unknown instruction 0x{:X}", value)),
            },
            _ => Err(format!("Unknown instruction 0x{:X}", value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_invalid_instruction() {
        let result = Instruction::try_from(0xF999);
        assert!(matches!(result, Err(message) if message == "Unknown instruction 0xF999"));
    }

    #[test]
    fn test_parse_valid_instruction() {
        let result = Instruction::try_from(0x2999);
        assert!(matches!(result, Ok(Instruction::Call { addr: 0x999 })));
    }
}
