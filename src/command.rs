use std::convert::TryFrom;

/// Chip-8 commands from Cowgod's Technical Reference
#[derive(Debug, PartialEq)]
pub enum Command {
    Cls,
    Ret,
    Jp(usize),
    Call(usize),
    SeV(usize, u8),
    SneV(usize, u8),
    SeVV(usize, usize),
    LdV(usize, u8),
    AddV(usize, u8),
    LdVV(usize, usize),
    OrVV(usize, usize),
    AndVV(usize, usize),
    XorVV(usize, usize),
    AddVV(usize, usize),
    SubVV(usize, usize),
    ShrVV(usize, usize),
    SubnVV(usize, usize),
    ShlVV(usize, usize),
    SneVV(usize, usize),
    LdI(usize),
    JpV0(usize),
    RndV(usize, u8),
    DrwVV(usize, usize, u8),
    SkpV(usize),
    SknpV(usize),
    LdVDt(usize),
    LdVK(usize),
    LdDtV(usize),
    LdStV(usize),
    AddIV(usize),
    LdFV(usize),
    LdBV(usize),
    LdIV(usize),
    LdVI(usize),
}

impl TryFrom<u16> for Command {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x00E0 => Ok(Self::Cls),
            0x00EE => Ok(Self::Ret),
            0x1000..=0x1FFF => {
                let addr = value & 0x0FFF;
                Ok(Self::Jp(addr as usize))
            }
            0x2000..=0x2FFF => {
                let addr = value & 0x0FFF;
                Ok(Self::Call(addr as usize))
            }
            0x3000..=0x3FFF => {
                let vx = value >> 8 & 0x0F;
                let byte = value & 0x00FF;
                Ok(Self::SeV(vx as usize, byte as u8))
            }
            0x4000..=0x4FFF => {
                let vx = value >> 8 & 0x0F;
                let byte = value & 0x00FF;
                Ok(Self::SneV(vx as usize, byte as u8))
            }
            0x5000..=0x5FFF if value % 0x10 == 0 => {
                let vx = value >> 8 & 0x0F;
                let vy = value >> 4 & 0x0F;
                Ok(Self::SeVV(vx as usize, vy as usize))
            }
            0x6000..=0x6FFF => {
                let vx = value >> 8 & 0x0F;
                let byte = value & 0x00FF;
                Ok(Self::LdV(vx as usize, byte as u8))
            }
            0x7000..=0x7FFF => {
                let vx = value >> 8 & 0x0F;
                let byte = value & 0x00FF;
                Ok(Self::AddV(vx as usize, byte as u8))
            }
            0x8000..=0x8FFF => {
                let vx = value >> 8 & 0x0F;
                let vy = value >> 4 & 0x0F;

                match value % 0x10 {
                    0x0 => Ok(Self::LdVV(vx as usize, vy as usize)),
                    0x1 => Ok(Self::OrVV(vx as usize, vy as usize)),
                    0x2 => Ok(Self::AndVV(vx as usize, vy as usize)),
                    0x3 => Ok(Self::XorVV(vx as usize, vy as usize)),
                    0x4 => Ok(Self::AddVV(vx as usize, vy as usize)),
                    0x5 => Ok(Self::SubVV(vx as usize, vy as usize)),
                    0x6 => Ok(Self::ShrVV(vx as usize, vy as usize)),
                    0x7 => Ok(Self::SubnVV(vx as usize, vy as usize)),
                    0xE => Ok(Self::ShlVV(vx as usize, vy as usize)),
                    _ => Err("Unknown command"),
                }
            }
            0x9000..=0x9FFF if value % 0x10 == 0 => {
                let vx = value >> 8 & 0x0F;
                let vy = value >> 4 & 0x0F;
                Ok(Self::SneVV(vx as usize, vy as usize))
            }
            0xA000..=0xAFFF => {
                let addr = value & 0x0FFF;
                Ok(Self::LdI(addr as usize))
            }
            0xB000..=0xBFFF => {
                let addr = value & 0x0FFF;
                Ok(Self::JpV0(addr as usize))
            }
            0xC000..=0xCFFF => {
                let vx = value >> 8 & 0x0F;
                let byte = value & 0x00FF;
                Ok(Self::RndV(vx as usize, byte as u8))
            }
            0xD000..=0xDFFF => {
                let vx = value >> 8 & 0x0F;
                let vy = value >> 4 & 0x0F;
                let nibble = value & 0x0F;
                Ok(Self::DrwVV(vx as usize, vy as usize, nibble as u8))
            }
            0xE000..=0xEFFF => {
                let vx = value >> 8 & 0x0F;

                match value & 0xFF {
                    0x9E => Ok(Self::SkpV(vx as usize)),
                    0xA1 => Ok(Self::SknpV(vx as usize)),
                    _ => Err("Unknown command"),
                }
            }
            0xF000..=0xFFFF => {
                let vx = value >> 8 & 0x0F;

                match value & 0xFF {
                    0x07 => Ok(Self::LdVDt(vx as usize)),
                    0x0A => Ok(Self::LdVK(vx as usize)),
                    0x15 => Ok(Self::LdDtV(vx as usize)),
                    0x18 => Ok(Self::LdStV(vx as usize)),
                    0x1E => Ok(Self::AddIV(vx as usize)),
                    0x29 => Ok(Self::LdFV(vx as usize)),
                    0x33 => Ok(Self::LdBV(vx as usize)),
                    0x55 => Ok(Self::LdIV(vx as usize)),
                    0x65 => Ok(Self::LdVI(vx as usize)),
                    _ => Err("Unknown command"),
                }
            }
            _ => Err("Unknown command"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_command_cls() {
        assert_eq!(Command::try_from(0x00E0), Ok(Command::Cls))
    }

    #[test]
    fn parse_command_ret() {
        assert_eq!(Command::try_from(0x00EE), Ok(Command::Ret))
    }

    #[test]
    fn parse_command_jp() {
        assert_eq!(Command::try_from(0x1234), Ok(Command::Jp(0x0234)))
    }

    #[test]
    fn parse_command_call() {
        assert_eq!(Command::try_from(0x2345), Ok(Command::Call(0x0345)))
    }

    #[test]
    fn parse_command_se_v() {
        assert_eq!(Command::try_from(0x3456), Ok(Command::SeV(4, 0x56)))
    }

    #[test]
    fn parse_command_sne_v() {
        assert_eq!(Command::try_from(0x4567), Ok(Command::SneV(5, 0x67)))
    }

    #[test]
    fn parse_command_se_vv() {
        assert_eq!(Command::try_from(0x5670), Ok(Command::SeVV(6, 7)))
    }

    #[test]
    fn parse_command_ld_v() {
        assert_eq!(Command::try_from(0x6789), Ok(Command::LdV(7, 0x89)))
    }

    #[test]
    fn parse_command_add_v() {
        assert_eq!(Command::try_from(0x7890), Ok(Command::AddV(8, 0x90)))
    }

    #[test]
    fn parse_command_ld_vv() {
        assert_eq!(Command::try_from(0x8900), Ok(Command::LdVV(9, 0)))
    }

    #[test]
    fn parse_command_or_vv() {
        assert_eq!(Command::try_from(0x8901), Ok(Command::OrVV(9, 0)))
    }

    #[test]
    fn parse_command_and_vv() {
        assert_eq!(Command::try_from(0x8902), Ok(Command::AndVV(9, 0)))
    }
}
