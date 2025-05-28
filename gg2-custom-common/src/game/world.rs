use crate::prelude::*;

pub trait World {
    type Players: Players;

    fn players(&self) -> &RwLock<Self::Players>;
}
