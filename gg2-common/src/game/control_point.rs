use crate::player::team::TeamSpawnable;

#[derive(Debug, Clone)]
pub struct RawControlPoint {
    pub team: TeamSpawnable,
    pub capturing_team: TeamSpawnable,
    pub capturing: u16,
}
