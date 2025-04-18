use super::io::error::{Error, Result};

pub mod mesh;

const BITMASK_SHIFT: u8 = 32;
const BITMASK_BITS_PER_BYTE: u8 = 6;
const BITMASK_MAX_BYTE: u8 = 2u8.pow(BITMASK_BITS_PER_BYTE as u32) - 1;

#[derive(Debug)]
pub struct WalkBitMask {
    pub mask: Vec<u8>,
    pub height: u16,
    pub width: u16,
}

impl WalkBitMask {
    pub fn read<'a, I>(stream: &mut I) -> Result<Self>
    where
        I: Iterator<Item = &'a str>,
    {
        let width = stream
            .next()
            .ok_or(Error::DataEOF)?
            .parse()
            .map_err(Error::ParseInt)?;
        let height = stream
            .next()
            .ok_or(Error::DataEOF)?
            .parse()
            .map_err(Error::ParseInt)?;
        let mask = stream
            .next()
            .ok_or(Error::DataEOF)?
            .chars()
            .map(Self::get_bitmask_from_character)
            .collect();

        Ok(Self {
            width,
            height,
            mask,
        })
    }

    /// Converts an ASCII character to a 6-bit mask.
    fn get_bitmask_from_character(character: char) -> u8 {
        (character as u8).saturating_sub(BITMASK_SHIFT) & BITMASK_MAX_BYTE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitmask_from_character_space() {
        assert_eq!(WalkBitMask::get_bitmask_from_character(' '), 0);
    }

    #[test]
    fn bitmask_from_character_back_slash() {
        assert_eq!(WalkBitMask::get_bitmask_from_character('\\'), 60);
    }

    #[test]
    fn bitmask_from_character_single_quote() {
        assert_eq!(WalkBitMask::get_bitmask_from_character('\''), 7);
    }

    #[test]
    fn bitmask_from_character_invalid() {
        assert_eq!(
            WalkBitMask::get_bitmask_from_character(0b0110_0000 as char),
            0
        );
    }
}
