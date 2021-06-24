//! Resources for managing players

use crate::player::components::PlayerComponent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type PlayersResource = HashMap<String, PlayerEntityData>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PlayerEntityData {
    pub player_component: PlayerComponent,
}
