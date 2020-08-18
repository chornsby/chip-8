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
}

impl TryFrom<u16> for Command {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x00E0 => Ok(Self::Cls),
            0x00EE => Ok(Self::Ret),
            0x1000..=0x1FFF => Ok(Self::Jp((value & 0x0FFF) as usize)),
            0x2000..=0x2FFF => Ok(Self::Call((value & 0x0FFF) as usize)),
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
            0x8000..=0x8FFF if value % 0x10 == 0 => {
                let vx = value >> 8 & 0x0F;
                let vy = value >> 4 & 0x0F;
                Ok(Self::LdVV(vx as usize, vy as usize))
            }
            0x8000..=0x8FFF if value % 0x10 == 1 => {
                let vx = value >> 8 & 0x0F;
                let vy = value >> 4 & 0x0F;
                Ok(Self::OrVV(vx as usize, vy as usize))
            }
            0x8000..=0x8FFF if value % 0x10 == 2 => {
                let vx = value >> 8 & 0x0F;
                let vy = value >> 4 & 0x0F;
                Ok(Self::AndVV(vx as usize, vy as usize))
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
