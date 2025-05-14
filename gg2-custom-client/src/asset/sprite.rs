use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteContextAsset {
    branch: SpriteContextBranch,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpriteContextBranch {
    Condition(Box<SpriteContextCondition>),
    Texture(AssetId),
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SpriteContextCondition {
    Team {
        #[serde(default)]
        red: Option<SpriteContextBranch>,
        #[serde(default)]
        blu: Option<SpriteContextBranch>,
        #[serde(default)]
        spectator: Option<SpriteContextBranch>,
    },
}

#[cfg(test)]
mod tests {
    use toml::toml;

    use super::*;

    #[test]
    fn serialize_root_texture() {
        let sprite = SpriteContextAsset {
            branch: SpriteContextBranch::Texture(AssetId::gg2("test/path")),
        };

        let expected = toml! {
            branch = "gg2:test/path"
        };

        let sprite_raw = toml::to_string(&sprite).unwrap();
        let expected_raw = toml::to_string(&expected).unwrap();

        assert_eq!(sprite_raw, expected_raw);
    }

    #[test]
    fn serialize_team() {
        let sprite = SpriteContextAsset {
            branch: SpriteContextBranch::Condition(Box::new(SpriteContextCondition::Team {
                red: Some(SpriteContextBranch::Texture(AssetId::gg2("red/test"))),
                blu: Some(SpriteContextBranch::Texture(AssetId::gg2("blu/test"))),
                spectator: Some(SpriteContextBranch::Texture(AssetId::gg2("spectator/test"))),
            })),
        };

        let expected = toml! {
            [branch]
            type = "team"
            red = "gg2:red/test"
            blu = "gg2:blu/test"
            spectator = "gg2:spectator/test"
        };

        let sprite_raw = toml::to_string(&sprite).unwrap();
        let expected_raw = toml::to_string(&expected).unwrap();

        assert_eq!(sprite_raw, expected_raw);
    }

    #[test]
    fn serialize_team_red_blu() {
        let sprite = SpriteContextAsset {
            branch: SpriteContextBranch::Condition(Box::new(SpriteContextCondition::Team {
                red: Some(SpriteContextBranch::Texture(AssetId::gg2("red/test"))),
                blu: Some(SpriteContextBranch::Texture(AssetId::gg2("blu/test"))),
                spectator: None,
            })),
        };

        let expected = toml! {
            [branch]
            type = "team"
            red = "gg2:red/test"
            blu = "gg2:blu/test"
        };

        let sprite_raw = toml::to_string(&sprite).unwrap();
        let expected_raw = toml::to_string(&expected).unwrap();

        assert_eq!(sprite_raw, expected_raw);
    }
}
