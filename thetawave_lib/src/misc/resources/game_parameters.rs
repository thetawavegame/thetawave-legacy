use serde::{Deserialize, Serialize};

/// Global game parameters
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct GameParametersResource {
    pub min_collision_knockback: f32,
}
