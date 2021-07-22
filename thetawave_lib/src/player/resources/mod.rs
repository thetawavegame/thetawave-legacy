//! Resources for managing players

use crate::player::components::PlayerComponent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Map names of characters to character data
pub type PlayersResource = HashMap<String, PlayerEntityData>;

/// Character data
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PlayerEntityData {
    /// Player component
    pub player_component: PlayerComponent,
}
