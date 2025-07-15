use std::collections::HashMap;

use serde::Deserialize;
use string_path::{SPath, SPathBuf};

use crate::prelude::*;

#[derive(Debug, Deserialize)]
pub struct AtlasDefinition {
    pub metadata: AtlasMetadata,
    pub selectors: Vec<AtlasSelector>,
}

impl AtlasDefinition {
    pub fn build(
        &self,
        textures: &HashMap<ResourceId, ImageBufferRGBA8>,
    ) -> Result<(TextureAtlas, ImageBufferRGBA8), AssetError> {
        let filtered_textures = textures
            .iter()
            .filter(|(id, _)| {
                self.selectors
                    .iter()
                    .any(|selector| selector.scan_check(id))
            })
            .map(|(id, sprite)| (id.clone(), sprite.clone()))
            .collect();

        TextureAtlas::new(self.metadata.size, filtered_textures)
    }
}

#[derive(Debug, Deserialize)]
pub struct AtlasMetadata {
    pub size: u32,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AtlasSelector {
    /// Selects all textures within a folder.
    Path { path: SPathBuf },
    /// Selects all textures within a folder under a specific namespace.
    NamespacePath { path: ResourceId },
    /// Selects one single texture.
    Single { asset: ResourceId },
}

impl AtlasSelector {
    fn scan_check(&self, value: &ResourceId) -> bool {
        match self {
            Self::Path { path } => Self::within_path(path, value.get_path()),
            Self::NamespacePath { path } => {
                (path.get_namespace() == value.get_namespace())
                    && Self::within_path(path.get_path(), value.get_path())
            }
            Self::Single { asset } => value == asset,
        }
    }

    fn within_path(path: &SPath, file: &SPath) -> bool {
        path.as_str().len() < file.as_str().len()
            && path != file // Prevent `test/path` from matching against `/test/path/`
            && path
                .iter()
                .zip(file)
                .all(|(path_part, file_part)| path_part == file_part)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selector_single() {
        let selector = AtlasSelector::Single {
            asset: ResourceId::gg2("test/texture"),
        };

        assert!(!selector.scan_check(&ResourceId::gg2("test")));
        assert!(!selector.scan_check(&ResourceId::gg2("test/texture/sub")));
        assert!(!selector.scan_check(&ResourceId::new("other", "test/texture")));
        assert!(selector.scan_check(&ResourceId::gg2("test/texture")));
    }

    #[test]
    fn selector_path() {
        let selector = AtlasSelector::Path {
            path: "test/path".into(),
        };

        assert!(!selector.scan_check(&ResourceId::gg2("test")));
        assert!(!selector.scan_check(&ResourceId::gg2("test/path")));
        assert!(!selector.scan_check(&ResourceId::gg2("test/path/")));
        assert!(selector.scan_check(&ResourceId::gg2("test/path/sub0")));
        assert!(selector.scan_check(&ResourceId::gg2("test/path/sub1")));
        assert!(selector.scan_check(&ResourceId::gg2("test/path/sub3")));
        assert!(selector.scan_check(&ResourceId::new("other", "test/path/sub0")));
    }
}
