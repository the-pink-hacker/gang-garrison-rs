use std::io::Read;

use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
    tasks::futures_lite::AsyncSeekExt,
};

use error::{Error, Result};
use flate2::bufread::ZlibDecoder;

use super::MapData;

pub mod error;

const PNG_SIGNATURE: &[u8] = b"\x89PNG\x0d\x0a\x1a\x0a";
const GG2_MAP_DATA_SIGNATURE: &[u8] = b"Gang Garrison 2 Level Data\x00";
const EXIF_CHUNK: &str = "zTXt";
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
                println!("{} {}", chunk_length, chunk_name);

                match &*chunk_name {
                    LAST_CHUNK => return Err(Error::EOF),
                    EXIF_CHUNK => {
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
    let mut buffer = vec![0; length as usize + CRC_LENGTH as usize];
    reader
        .read_exact(&mut buffer)
        .await
        .map_err(Error::ReadIO)?;

    if !buffer.starts_with(GG2_MAP_DATA_SIGNATURE) {
        return Err(Error::ChunkFormat);
    }

    match (*buffer.get(GG2_MAP_DATA_SIGNATURE.len()).ok_or(Error::EOF)?).into() {
        MapCompression::Deflate => {
            let mut decoder = ZlibDecoder::new(&buffer[(GG2_MAP_DATA_SIGNATURE.len() + 1)..]);
            let mut map_data_buffer = Vec::with_capacity(length as usize);
            decoder
                .read_to_end(&mut map_data_buffer)
                .map_err(Error::ReadIO)?;
            Ok(map_data_buffer)
        }
        MapCompression::Unknown => Err(Error::CompressionType(MapCompression::Unknown)),
    }
}
