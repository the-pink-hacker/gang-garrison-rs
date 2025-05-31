use std::str::FromStr;

use crate::{gamemode::Gamemode, player::team::TeamSpawnable};

use super::{data::MapData, entity::MapEntity};
use error::{MapIoError, Result};
use glam::Vec2;

pub mod error;

pub const DATA_HEADER_KEYWORD: &str = "Gang Garrison 2 Level Data";

const BITMASK_SHIFT: u8 = b' ';
const BITMASK_BITS_PER_BYTE: u8 = 6;
const BITMASK_MAX_BYTE: u8 = 2u8.pow(BITMASK_BITS_PER_BYTE as u32) - 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MapDataTag {
    Entities,
    EndEntities,
    WalkMask,
    EndWalkMask,
}

impl MapDataTag {
    pub fn expect_end_entities(self) -> Result<()> {
        if self == Self::EndEntities {
            Ok(())
        } else {
            Err(MapIoError::DataTagUnclosed {
                expected: Self::EndEntities,
                got: self,
            })
        }
    }

    pub fn expect_end_walk_mask(self) -> Result<()> {
        if self == Self::EndWalkMask {
            Ok(())
        } else {
            Err(MapIoError::DataTagUnclosed {
                expected: Self::EndWalkMask,
                got: self,
            })
        }
    }
}

impl std::fmt::Display for MapDataTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Self::Entities => "{ENTITIES}",
            Self::EndEntities => "{END ENTITIES}",
            Self::WalkMask => "{WALKMASK}",
            Self::EndWalkMask => "{END WALKMASK}",
        };

        f.write_str(output)
    }
}

impl FromStr for MapDataTag {
    type Err = MapIoError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "{ENTITIES}" => Ok(Self::Entities),
            "{END ENTITIES}" => Ok(Self::EndEntities),
            "{WALKMASK}" => Ok(Self::WalkMask),
            "{END WALKMASK}" => Ok(Self::EndWalkMask),
            _ => Err(MapIoError::DataTag(s.to_string())),
        }
    }
}

impl MapData {
    pub fn load_from_memory(buf: &[u8]) -> Result<Self> {
        let data_text = png::Decoder::new(buf)
            .read_info()?
            .info()
            .compressed_latin1_text
            .iter()
            .find(|chunk| chunk.keyword == DATA_HEADER_KEYWORD)
            .ok_or(MapIoError::Chunk)?
            .get_text()?;

        let mut data_lines = data_text.lines();

        let mut entities = None;
        let mut walk_mask = None;

        while let Some(data_line) = data_lines.next() {
            let tag = MapDataTag::from_str(data_line)?;

            match tag {
                MapDataTag::Entities => {
                    entities = Some(Self::read_entities(&mut data_lines)?);
                }
                MapDataTag::WalkMask => {
                    walk_mask = Some(Self::read_walk_mask(&mut data_lines)?);
                }
                MapDataTag::EndEntities | MapDataTag::EndWalkMask => {
                    return Err(MapIoError::DataTagUnexpected(tag));
                }
            }
        }

        let entities = entities.ok_or(MapIoError::DataTagMissing(MapDataTag::Entities))?;
        let walk_mask = walk_mask.ok_or(MapIoError::DataTagMissing(MapDataTag::Entities))?;

        let gamemode = Gamemode::scan_map_entities(&entities)?;

        let mut blu_spawns = <[Vec<Vec2>; 5]>::default();
        let mut red_spawns = <[Vec<Vec2>; 5]>::default();

        for entity in entities {
            let (group, position, team) = match entity {
                MapEntity::BluSpawn0(position) => (0, position, TeamSpawnable::Blu),
                MapEntity::BluSpawn1(position) => (1, position, TeamSpawnable::Blu),
                MapEntity::BluSpawn2(position) => (2, position, TeamSpawnable::Blu),
                MapEntity::BluSpawn3(position) => (3, position, TeamSpawnable::Blu),
                MapEntity::BluSpawn4(position) => (4, position, TeamSpawnable::Blu),
                MapEntity::RedSpawn0(position) => (0, position, TeamSpawnable::Red),
                MapEntity::RedSpawn1(position) => (1, position, TeamSpawnable::Red),
                MapEntity::RedSpawn2(position) => (2, position, TeamSpawnable::Red),
                MapEntity::RedSpawn3(position) => (3, position, TeamSpawnable::Red),
                MapEntity::RedSpawn4(position) => (4, position, TeamSpawnable::Red),
                _ => continue,
            };

            let spawns = match team {
                TeamSpawnable::Red => &mut red_spawns,
                TeamSpawnable::Blu => &mut blu_spawns,
            };

            spawns[group].push(position.into());
        }

        Ok(Self {
            walk_mask,
            blu_spawns,
            red_spawns,
            gamemode,
        })
    }

    fn read_entities<'a, I: Iterator<Item = &'a str>>(
        data_lines: &mut I,
    ) -> Result<Vec<MapEntity>> {
        let entities_raw = data_lines.next().ok_or(MapIoError::DataEOF)?;

        data_lines
            .next()
            .ok_or(MapIoError::DataEOF)
            .and_then(MapDataTag::from_str)
            .and_then(MapDataTag::expect_end_entities)?;

        // TODO: Implement custom ggson parser
        let entities_hjson = entities_raw.replace(",", "\n").replace("}", "\n}");

        Ok(serde_hjson::from_str(&entities_hjson)?)
    }

    fn read_walk_mask<'a, I: Iterator<Item = &'a str>>(data_lines: &mut I) -> Result<Vec<u8>> {
        let _width = data_lines
            .next()
            .ok_or(MapIoError::DataEOF)?
            .parse::<u32>()?;
        let _height = data_lines
            .next()
            .ok_or(MapIoError::DataEOF)?
            .parse::<u32>()?;
        let walk_mask_raw = data_lines.next().ok_or(MapIoError::DataEOF)?;

        //dbg!(walkmask_raw);

        data_lines
            .next()
            .ok_or(MapIoError::DataEOF)
            .and_then(MapDataTag::from_str)
            .and_then(MapDataTag::expect_end_walk_mask)?;

        Ok(walk_mask_raw
            .bytes()
            .map(get_bitmask_from_character)
            .collect())
    }
}

/// Converts an ASCII character to a 6-bit mask.
fn get_bitmask_from_character(character: u8) -> u8 {
    character.saturating_sub(BITMASK_SHIFT) & BITMASK_MAX_BYTE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_tag_entities() {
        let parsed = MapDataTag::from_str("{ENTITIES}").unwrap();
        assert_eq!(parsed, MapDataTag::Entities);
    }

    #[test]
    fn data_tag_end_entities() {
        let parsed = MapDataTag::from_str("{END ENTITIES}").unwrap();
        assert_eq!(parsed, MapDataTag::EndEntities);
    }

    #[test]
    fn data_tag_walk_mask() {
        let parsed = MapDataTag::from_str("{WALKMASK}").unwrap();
        assert_eq!(parsed, MapDataTag::WalkMask);
    }

    #[test]
    fn data_tag_end_walk_mask() {
        let parsed = MapDataTag::from_str("{END WALKMASK}").unwrap();
        assert_eq!(parsed, MapDataTag::EndWalkMask);
    }

    #[test]
    fn data_tag_error() {
        assert!(MapDataTag::from_str("super secret tag").is_err());
    }
}
