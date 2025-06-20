use crate::prelude::*;

impl SpriteRenderable for Player {
    fn get_context_id() -> ResourceId {
        ResourceId::gg2("player")
    }

    #[inline]
    fn get_transform(&self) -> Transform {
        self.transform
    }

    #[inline]
    fn get_team(&self) -> Option<Team> {
        Some(self.team)
    }

    #[inline]
    fn get_class(&self) -> Option<ClassGeneric> {
        Some(self.class)
    }
}

#[derive(Debug, Default)]
pub struct ClientPlayers {
    players: Vec<Player>,
    client_player: Option<PlayerId>,
}

impl ClientPlayers {
    #[inline]
    pub fn set_client_player(&mut self, id: PlayerId) {
        self.client_player = Some(id);
    }

    pub fn get_client(&self) -> Result<&Player, ClientError> {
        let id = self.client_player.ok_or(ClientError::ClientPlayerLookup)?;
        Ok(self.get(id)?)
    }

    pub fn get_client_mut(&mut self) -> Result<&mut Player, ClientError> {
        let id = self.client_player.ok_or(ClientError::ClientPlayerLookup)?;
        Ok(self.get_mut(id)?)
    }
}

impl Players for ClientPlayers {
    #[inline]
    fn as_vec(&self) -> &Vec<Player> {
        &self.players
    }

    #[inline]
    fn as_vec_mut(&mut self) -> &mut Vec<Player> {
        &mut self.players
    }

    #[inline]
    fn into_vec(self) -> Vec<Player> {
        self.players
    }
}

impl IntoIterator for ClientPlayers {
    type Item = Player;
    type IntoIter = std::vec::IntoIter<Player>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.players.into_iter()
    }
}

impl<'a> IntoIterator for &'a ClientPlayers {
    type Item = &'a Player;
    type IntoIter = std::slice::Iter<'a, Player>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.players.iter()
    }
}

impl<'a> IntoIterator for &'a mut ClientPlayers {
    type Item = &'a mut Player;
    type IntoIter = std::slice::IterMut<'a, Player>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.players.iter_mut()
    }
}
