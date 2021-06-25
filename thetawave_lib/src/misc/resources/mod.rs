//! Miscellaneous resources

use amethyst::renderer::palette::Srgba;
use serde::{Deserialize, Serialize};

mod game_parameters;

pub use self::game_parameters::GameParametersResource;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct DebugLinesConfig {
    pub line_width: f32,
    pub hitbox_color: Srgba,
    pub item_attractor_color: Srgba,
    pub consumable_attractor_color: Srgba,
    pub blast_attractor_color: Srgba,
}
