mod sprite;

pub use self::{
    sprite::{SpriteResource},
};

use crate::components::{Enemy, Item, Consumable};

pub type EnemyPool = std::collections::HashMap<String, Enemy>;
pub type ItemPool = std::collections::HashMap<String, Item>;
pub type ConsumablePool = std::collections::HashMap<String, Consumable>;