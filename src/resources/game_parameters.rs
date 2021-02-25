use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct GameParametersResource {
    pub min_collision_knockback: f32,
}
