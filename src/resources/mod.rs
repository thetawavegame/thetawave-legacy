use crate::components::{
    Animation, AutoFireComponent, BlasterComponent, Consumable, Enemy, HealthComponent,
    Hitbox2DComponent, ItemComponent, Motion2DComponent,
};
use serde::{Deserialize, Serialize};

mod sprite;

pub use self::sprite::{initialize_sprite_resource, SpriteResource};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EnemyEntityData {
    pub animation_component: Animation,
    pub enemy_component: Enemy,
    pub hitbox_component: Hitbox2DComponent,
    pub blaster_component: Option<BlasterComponent>,
    pub autofire_component: Option<AutoFireComponent>,
    pub motion2d_component: Motion2DComponent,
    pub health_component: HealthComponent,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ItemEntityData {
    pub item_component: ItemComponent,
    pub animation_component: Option<Animation>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConsumableEntityData {
    pub consumable_component: Consumable,
    pub hitbox_component: Hitbox2DComponent,
}

pub type EnemyPool = std::collections::HashMap<String, EnemyEntityData>;
pub type ItemPool = std::collections::HashMap<String, ItemEntityData>;
pub type ConsumablePool = std::collections::HashMap<String, ConsumableEntityData>;
