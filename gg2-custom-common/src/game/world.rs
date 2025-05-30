use crate::prelude::*;

#[allow(async_fn_in_trait)]
pub trait World {
    type Players: Players;

    fn players(&self) -> &RwLock<Self::Players>;
}
