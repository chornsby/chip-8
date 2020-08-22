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

    macro_rules! parse_command_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $value;
                    assert_eq!(Command::try_from(input), Ok(expected));
                }
            )*
        };
    }

    parse_command_tests! {
        cls: (0x00E0, Command::Cls),
        ret: (0x00EE, Command::Ret),
        jp: (0x1234, Command::Jp(0x0234)),
        call: (0x2345, Command::Call(0x0345)),
        se_v: (0x3456, Command::SeV(0x4, 0x56)),
        sne_v: (0x4567, Command::SneV(0x5, 0x67)),
        se_vv: (0x5670, Command::SeVV(0x6, 0x7)),
        ld_v: (0x6789, Command::LdV(0x7, 0x89)),
        add_v: (0x789A, Command::AddV(0x8, 0x9A)),
        ld_vv: (0x89A0, Command::LdVV(0x9, 0xA)),
        or_vv: (0x89A1, Command::OrVV(0x9, 0xA)),
        and_vv: (0x89A2, Command::AndVV(0x9, 0xA)),
        xor_vv: (0x89A3, Command::XorVV(0x9, 0xA)),
        add_vv: (0x89A4, Command::AddVV(0x9, 0xA)),
        sub_vv: (0x89A5, Command::SubVV(0x9, 0xA)),
        shr_vv: (0x89A6, Command::ShrVV(0x9, 0xA)),
        subn_vv: (0x89A7, Command::SubnVV(0x9, 0xA)),
        shl_vv: (0x89AE, Command::ShlVV(0x9, 0xA)),
        sne_vv: (0x9AB0, Command::SneVV(0xA, 0xB)),
        ld_i: (0xABCD, Command::LdI(0x0BCD)),
        jp_v0: (0xBCDE, Command::JpV0(0xCDE)),
        rnd_v: (0xCDEF, Command::RndV(0xD, 0xEF)),
        drw_vv: (0xDEF0, Command::DrwVV(0xE, 0xF, 0x0)),
        skp_v: (0xE09E, Command::SkpV(0x0)),
        sknp_v: (0xE1A1, Command::SknpV(0x1)),
        ld_v_dt: (0xF207, Command::LdVDt(0x2)),
        ld_v_k: (0xF30A, Command::LdVK(0x3)),
        ld_dt_v: (0xF415, Command::LdDtV(0x4)),
        ld_st_v: (0xF518, Command::LdStV(0x5)),
        add_i_v: (0xF61E, Command::AddIV(0x6)),
        ld_f_v: (0xF729, Command::LdFV(0x7)),
        ld_b_v: (0xF833, Command::LdBV(0x8)),
        ld_i_v: (0xF955, Command::LdIV(0x9)),
        ld_v_i: (0xFA65, Command::LdVI(0xA)),
    }
}
