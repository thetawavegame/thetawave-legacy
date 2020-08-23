use crate::components::{Consumable, Enemy, Item};

mod sprite;

pub use self::sprite::{initialize_sprite_resource, SpriteResource};

pub type EnemyPool = std::collections::HashMap<String, Enemy>;
pub type ItemPool = std::collections::HashMap<String, Item>;
pub type ConsumablePool = std::collections::HashMap<String, Consumable>;
