use std::str::FromStr;

use serde::{Deserialize, Serialize, de::Visitor};

use crate::prelude::*;

impl Serialize for ResourceId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ResourceId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(AssetIdVisitor)
    }
}

struct AssetIdVisitor;

impl Visitor<'_> for AssetIdVisitor {
    type Value = ResourceId;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("A namespaced asset identifier")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        ResourceId::from_str(v).map_err(E::custom)
    }
}
