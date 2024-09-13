use std::io::Read;

use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
    tasks::futures_lite::AsyncSeekExt,
};

use error::{Error, Result};
use flate2::{bufread::ZlibDecoder, Crc, CrcReader};

use super::MapData;

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

#[derive(Default)]
pub struct MapDataLoader;

impl AssetLoader for MapDataLoader {
    type Asset = MapData;
    type Settings = ();
    type Error = Error;

    async fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader<'_>,
        settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset> {
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

            println!("{}", map_data_buffer.escape_ascii());
            todo!();
        } else {
            Err(Error::IncorrectFormat)
        }
    }
}

async fn is_png(reader: &mut bevy::asset::io::Reader<'_>) -> Result<bool> {
    let mut signature = [0; 8];
    reader
        .read_exact(&mut signature)
        .await
        .map_err(Error::ReadIO)?;
    Ok(signature == PNG_SIGNATURE)
}

async fn get_png_chunk_header(reader: &mut bevy::asset::io::Reader<'_>) -> Result<(u32, String)> {
    let mut raw_header = [0; 8];
    reader
        .read_exact(&mut raw_header)
        .await
        .map_err(Error::ReadIO)?;

    let chunk_length = u32::from_be_bytes(raw_header[..4].try_into().unwrap());
    let chunk_name = String::from_utf8(raw_header[4..].to_vec()).map_err(|_| Error::ChunkFormat)?;

    Ok((chunk_length, chunk_name))
}

async fn read_map_data_chunk(
    reader: &mut bevy::asset::io::Reader<'_>,
    length: u32,
) -> Result<Vec<u8>> {
    let mut buffer = vec![0; length as usize];
    buffer[0] = 0;
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
                Ok(map_data_buffer)
            } else {
                Err(Error::CorruptedData(expected_crc, crc.sum()))
            }
        }
        MapCompression::Unknown => Err(Error::CompressionType(MapCompression::Unknown)),
    }
}
