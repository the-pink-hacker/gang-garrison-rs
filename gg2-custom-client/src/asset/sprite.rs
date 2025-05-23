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

impl SpriteContextBranch {
    fn evaluate<R: SpriteRenderable + ?Sized>(&self, renderable: &R) -> Option<&AssetId> {
        match self {
            Self::Condition(condition) => condition.evaluate(renderable),
            Self::Texture(id) => Some(id),
        }
    }
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

impl SpriteContextCondition {
    fn evaluate<R: SpriteRenderable + ?Sized>(&self, renderable: &R) -> Option<&AssetId> {
        match self {
            Self::Team {
                red,
                blu,
                spectator,
            } => renderable
                .get_team()
                .map(|team| match team {
                    Team::Red => red,
                    Team::Blu => blu,
                    Team::Spectator => spectator,
                })
                .and_then(Option::as_ref),
        }
        .and_then(|branch| branch.evaluate(renderable))
    }
}

const ORIGIN_CENTER: Vec2 = Vec2::new(0.5, 0.5);

pub trait SpriteRenderable {
    fn render(
        &self,
        atlas: &TextureAtlas,
        asset_server: &AssetServer,
    ) -> Result<Option<SpriteInstance>, ClientError> {
        let sprite_context = asset_server.get_sprite(&Self::get_context_id())?;

        if let Some(id) = sprite_context.branch.evaluate(self) {
            let texture_uv = atlas.lookup_sprite(id)?;

            Ok(Some(SpriteInstance::from_transform_origin(
                self.get_transform(),
                ORIGIN_CENTER,
                texture_uv,
            )))
        } else {
            Ok(None)
        }
    }

    fn get_context_id() -> AssetId;

    fn get_transform(&self) -> Transform;

    fn get_team(&self) -> Option<Team>;
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
