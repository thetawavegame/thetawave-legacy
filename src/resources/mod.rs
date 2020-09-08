use crate::components::{Animation, Consumable, Enemy, Hitbox2DComponent, Motion2DComponent, Item};
use serde::{Deserialize, Serialize};

mod sprite;

pub use self::sprite::{initialize_sprite_resource, SpriteResource};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EnemyEntityData {
    pub animation_component: Animation,
    pub enemy_component: Enemy,
    pub hitbox_component: Hitbox2DComponent,
    pub motion2d_component: Motion2DComponent,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ItemEntityData {
    pub item_component: Item,
    pub hitbox_component: Hitbox2DComponent,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConsumableEntityData {
    pub consumable_component: Consumable,
    pub hitbox_component: Hitbox2DComponent,
}

pub type EnemyPool = std::collections::HashMap<String, EnemyEntityData>;
pub type ItemPool = std::collections::HashMap<String, ItemEntityData>;
pub type ConsumablePool = std::collections::HashMap<String, ConsumableEntityData>;
