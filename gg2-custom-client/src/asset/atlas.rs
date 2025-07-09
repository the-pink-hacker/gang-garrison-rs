use serde::Deserialize;
use string_path::SPathBuf;

#[derive(Debug, Deserialize)]
pub struct AtlasDefinition {
    pub metadata: AtlasMetadata,
    pub selector: AtlasSelector,
}

#[derive(Debug, Deserialize)]
pub struct AtlasMetadata {
    pub size: u32,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AtlasSelector {
    Path { path: SPathBuf },
    Union { values: Vec<AtlasSelector> },
}
