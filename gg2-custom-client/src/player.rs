use crate::prelude::*;

#[derive(Debug, Default)]
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
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct Players {
    players: Vec<Player>,
    client_player: Option<PlayerId>,
}

impl Players {
    pub fn player_join(&mut self, player: Player) -> Result<PlayerId, CommonError> {
        let next_id = self.next_id()?;

        self.players.push(player);

        Ok(next_id)
    }

    pub fn next_id(&self) -> Result<PlayerId, CommonError> {
        PlayerId::try_from(self.players.len())
    }

    #[inline]
    pub fn len(&self) -> u8 {
        self.players.len() as u8
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.players.is_empty()
    }

    #[inline]
    pub fn set_client_player(&mut self, id: PlayerId) {
        self.client_player = Some(id);
    }

    pub fn get(&self, id: PlayerId) -> Result<&Player, CommonError> {
        self.players
            .get(usize::from(id))
            .ok_or(CommonError::PlayerLookup(id))
    }

    pub fn get_mut(&mut self, id: PlayerId) -> Result<&mut Player, CommonError> {
        self.players
            .get_mut(usize::from(id))
            .ok_or(CommonError::PlayerLookup(id))
    }

    pub fn get_client(&self) -> Result<&Player, ClientError> {
        let id = self.client_player.ok_or(ClientError::ClientPlayerLookup)?;
        Ok(self.get(id)?)
    }

    pub fn get_client_mut(&mut self) -> Result<&mut Player, ClientError> {
        let id = self.client_player.ok_or(ClientError::ClientPlayerLookup)?;
        Ok(self.get_mut(id)?)
    }

    pub fn remove(&mut self, id: PlayerId) -> Result<Player, CommonError> {
        if u8::from(id) < self.len() {
            Ok(self.players.remove(usize::from(id)))
        } else {
            Err(CommonError::PlayerLookup(id))
        }
    }
}
