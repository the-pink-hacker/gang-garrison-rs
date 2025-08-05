use crate::prelude::*;

pub const PLAYER_SCALE: f32 = 64.0;

#[derive(Debug, Default, Clone)]
pub struct Player {
    pub name: GGStringShort,
    pub transform: Transform,
    pub velocity: Vec2,
    pub input_state: RawInput,
    pub class: ClassGeneric,
    pub team: Team,
}

impl Player {
    pub fn from_name(name: GGStringShort) -> Self {
        Self {
            name,
            transform: Transform {
                scale: Vec2::splat(PLAYER_SCALE),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub trait Players {
    fn as_vec(&self) -> &Vec<Player>;

    fn as_vec_mut(&mut self) -> &mut Vec<Player>;

    fn into_vec(self) -> Vec<Player>;

    #[inline]
    fn len(&self) -> u8 {
        self.as_vec().len() as u8
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.as_vec().is_empty()
    }

    #[inline]
    fn iter(&self) -> std::slice::Iter<'_, Player> {
        self.as_vec().iter()
    }

    #[inline]
    fn iter_mut(&mut self) -> std::slice::IterMut<'_, Player> {
        self.as_vec_mut().iter_mut()
    }

    fn get(&self, id: PlayerId) -> Result<&Player, CommonError> {
        self.as_vec()
            .get(usize::from(id))
            .ok_or(CommonError::PlayerLookup(id))
    }

    fn get_mut(&mut self, id: PlayerId) -> Result<&mut Player, CommonError> {
        self.as_vec_mut()
            .get_mut(usize::from(id))
            .ok_or(CommonError::PlayerLookup(id))
    }

    fn next_id(&self) -> Result<PlayerId, CommonError> {
        PlayerId::try_from(self.as_vec().len())
    }

    fn push(&mut self, player: Player) -> Result<PlayerId, CommonError> {
        let next_id = self.next_id()?;

        self.as_vec_mut().push(player);

        Ok(next_id)
    }

    fn remove(&mut self, id: PlayerId) -> Result<Player, CommonError> {
        if u8::from(id) < self.len() {
            Ok(self.as_vec_mut().remove(usize::from(id)))
        } else {
            Err(CommonError::PlayerLookup(id))
        }
    }
}

pub trait PlayersIter<'a> {
    fn flat_zip_mut<T, I>(&'a mut self, iterator: I) -> impl Iterator<Item = (&'a mut Player, T)>
    where
        I: IntoIterator<Item = Option<T>>;
}

impl<'a, P: Players + ?Sized> PlayersIter<'a> for P {
    #[inline]
    fn flat_zip_mut<T, I>(&'a mut self, iterator: I) -> impl Iterator<Item = (&'a mut Player, T)>
    where
        I: IntoIterator<Item = Option<T>>,
    {
        self.as_vec_mut()
            .iter_mut()
            .zip(iterator)
            .flat_map(|(player, item)| item.map(|item| (player, item)))
    }
}
