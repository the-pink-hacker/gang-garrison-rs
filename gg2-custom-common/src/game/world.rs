use crate::prelude::*;

pub trait World {
    fn players(&self) -> &RwLock<dyn Players + Send + Sync>;
}
