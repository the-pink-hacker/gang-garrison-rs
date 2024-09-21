use std::{fmt::Display, io::Read, str::FromStr};

use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    tasks::futures_lite::AsyncSeekExt,
};

use error::{Error, Result};
use flate2::{bufread::ZlibDecoder, Crc, CrcReader};

use crate::entity::entities::MapEntity;

use super::{
    collision::{mesh::WalkQuadMask, WalkBitMask},
    MapData,
};

pub mod error;

const PNG_SIGNATURE: &[u8] = b"\x89PNG\x0d\x0a\x1a\x0a";
const GG2_MAP_DATA_SIGNATURE: &[u8] = b"Gang Garrison 2 Level Data\x00";
const DATA_CHUNK: &str = "zTXt";
const LAST_CHUNK: &str = "IEND";
const CRC_LENGTH: u8 = 4;

#[derive(Debug)]
pub enum MapCompression {
    Deflate,
    Unknown,
}

impl From<u8> for MapCompression {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Deflate,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MapDataTag {
    Entities,
    EndEntities,
    WalkMask,
    EndWalkMask,
}

impl Display for MapDataTag {
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
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "{ENTITIES}" => Ok(Self::Entities),
            "{END ENTITIES}" => Ok(Self::EndEntities),
            "{WALKMASK}" => Ok(Self::WalkMask),
            "{END WALKMASK}" => Ok(Self::EndWalkMask),
            _ => Err(Error::DataTag(s.to_string())),
        }
    }
}

#[derive(Default)]
pub struct MapDataLoader;

impl AssetLoader for MapDataLoader {
    type Asset = MapData;
    type Settings = ();
    type Error = Error;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<MapData> {
        if is_png(reader).await? {
            let map_data_buffer = loop {
                let (chunk_length, chunk_name) = get_png_chunk_header(reader).await?;

                match &*chunk_name {
                    LAST_CHUNK => return Err(Error::EOF),
                    DATA_CHUNK => {
                        break read_map_data_chunk(reader, chunk_length).await?;
                    }
                    _ => {
                        reader
                            .seek(std::io::SeekFrom::Current(
                                chunk_length as i64 + CRC_LENGTH as i64,
                            ))
                            .await
                            .map_err(Error::ReadIO)?;
                    }
                }
            };

            parse_map_data(map_data_buffer)
        } else {
            Err(Error::IncorrectFormat)
        }
    }
}

async fn is_png<'a>(reader: &'a mut Reader<'a>) -> Result<bool> {
    let mut signature = [0; 8];
    reader
        .read_exact(&mut signature)
        .await
        .map_err(Error::ReadIO)?;
    Ok(signature == PNG_SIGNATURE)
}

async fn get_png_chunk_header<'a>(reader: &'a mut Reader<'a>) -> Result<(u32, String)> {
    let mut raw_header = [0; 8];
    reader
        .read_exact(&mut raw_header)
        .await
        .map_err(Error::ReadIO)?;

    let chunk_length = u32::from_be_bytes(raw_header[..4].try_into().unwrap());
    let chunk_name = String::from_utf8(raw_header[4..].to_vec()).map_err(|_| Error::ChunkFormat)?;

    Ok((chunk_length, chunk_name))
}

async fn read_map_data_chunk<'a>(reader: &'a mut Reader<'a>, length: u32) -> Result<String> {
    let mut buffer = vec![0; length as usize];

    reader
        .read_exact(&mut buffer)
        .await
        .map_err(Error::ReadIO)?;

    let mut crc_reader = CrcReader::new(&buffer[..]);

    let mut signature = [0; GG2_MAP_DATA_SIGNATURE.len() + 1];
    crc_reader
        .read_exact(&mut signature)
        .map_err(Error::ReadIO)?;

    if signature[0..GG2_MAP_DATA_SIGNATURE.len()] != *GG2_MAP_DATA_SIGNATURE {
        return Err(Error::ChunkFormat);
    }

    match signature[27].into() {
        MapCompression::Deflate => {
            let mut decoder = ZlibDecoder::new(&mut crc_reader);
            let mut map_data_buffer = Vec::with_capacity(length as usize);
            decoder
                .read_to_end(&mut map_data_buffer)
                .map_err(Error::ReadIO)?;

            let mut crc_buffer = [0; 4];
            reader
                .read_exact(&mut crc_buffer)
                .await
                .map_err(Error::ReadIO)?;
            let expected_crc = u32::from_be_bytes(crc_buffer);
            let mut crc = Crc::new();
            crc.update(DATA_CHUNK.as_bytes());
            crc.combine(crc_reader.crc());

            if crc.sum() == expected_crc {
                String::from_utf8(map_data_buffer).map_err(|_| Error::ChunkFormat)
            } else {
                Err(Error::CorruptedData(expected_crc, crc.sum()))
            }
        }
        MapCompression::Unknown => Err(Error::CompressionType(MapCompression::Unknown)),
    }
}

fn parse_map_data(raw: String) -> Result<MapData> {
    let mut raw_lines = raw.lines();

    let mut entities = None;
    let mut walk_mask = None;

    while let Some(tag_raw) = raw_lines.next() {
        let tag = MapDataTag::from_str(tag_raw)?;

        match tag {
            MapDataTag::Entities => {
                entities = Some(parse_map_entities(&mut raw_lines)?);
            }
            MapDataTag::WalkMask => {
                walk_mask = Some(WalkBitMask::read(&mut raw_lines)?);
            }
            MapDataTag::EndEntities | MapDataTag::EndWalkMask => (),
        }
    }

    let walk_bit_mask = walk_mask.ok_or(Error::DataTagMissing(MapDataTag::WalkMask))?;
    let walk_mask = WalkQuadMask::from_bits(walk_bit_mask).triangulate();

    Ok(MapData {
        entities: entities.ok_or(Error::DataTagMissing(MapDataTag::Entities))?,
        walk_mask,
    })
}

fn parse_map_entities<'a, I>(data_stream: &mut I) -> Result<Vec<MapEntity>>
where
    I: Iterator<Item = &'a str>,
{
    let data = data_stream
        .next()
        .ok_or(Error::DataEOF)?
        .replace(',', "\n")
        .replace('}', "\n}");
    serde_hjson::from_str(&data).map_err(Error::Entity)
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
