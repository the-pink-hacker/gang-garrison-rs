use core::panic;

use super::io::error::{Error, Result};

const BITMASK_SHIFT: u8 = 32;
const BITMASK_BITS_PER_BYTE: u8 = 6;
const BITMASK_MAX_BYTE: u8 = 2u8.pow(BITMASK_BITS_PER_BYTE as u32) - 1;

#[derive(Debug)]
pub struct WalkBitMask {
    mask: Vec<u8>,
    height: u16,
    width: u16,
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

    pub fn collidable(&self, x: u16, y: u16) -> bool {
        if x >= self.width || y >= self.height {
            return true;
        }

        let bit_index = x as u32 + y as u32 * self.width as u32;
        let byte_index = bit_index / BITMASK_BITS_PER_BYTE as u32;
        let byte_bit_index = bit_index % BITMASK_BITS_PER_BYTE as u32;

        self.mask
            .get(byte_index as usize)
            .map(|byte| byte & 2u8.pow(byte_bit_index) != 0)
            .unwrap_or(true)
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

    const BITMASK: [u8; 4] = [0b0000_0001, 0b0000_0001, 0b0000_0000, 0b0000_0000];

    #[test]
    fn bitmask_lookup_0() {
        let bitmask = WalkBitMask {
            width: 8,
            height: 4,
            mask: BITMASK.into(),
        };

        assert!(bitmask.collidable(0, 0));
    }

    #[test]
    fn bitmask_lookup_1() {
        let bitmask = WalkBitMask {
            width: 8,
            height: 4,
            mask: BITMASK.into(),
        };

        assert!(bitmask.collidable(6, 0));
    }

    #[test]
    fn bitmask_lookup_out_of_bounds_x() {
        let bitmask = WalkBitMask {
            width: 4,
            height: 2,
            mask: BITMASK.into(),
        };

        assert!(bitmask.collidable(4, 1));
    }

    #[test]
    fn bitmask_lookup_out_of_bounds_y() {
        let bitmask = WalkBitMask {
            width: 2,
            height: 4,
            mask: BITMASK.into(),
        };

        assert!(bitmask.collidable(0, 4));
    }
}
