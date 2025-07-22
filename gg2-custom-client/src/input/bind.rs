use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InputAxisBind(Vec<InputAxisCode>);

impl From<Vec<InputAxisCode>> for InputAxisBind {
    fn from(value: Vec<InputAxisCode>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InputButtonBind(Vec<InputButtonCode>);

impl From<Vec<InputButtonCode>> for InputButtonBind {
    fn from(value: Vec<InputButtonCode>) -> Self {
        Self(value)
    }
}

impl IntoIterator for InputButtonBind {
    type Item = InputButtonCode;
    type IntoIter = std::vec::IntoIter<InputButtonCode>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
