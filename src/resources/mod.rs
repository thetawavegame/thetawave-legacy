use crate::components::{
    AnimationComponent, AutoFireComponent, BlasterComponent, ConsumableComponent, EnemyComponent,
    HealthComponent, Hitbox2DComponent, ItemComponent, Motion2DComponent,
};
use amethyst::renderer::palette::Srgba;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod sprite;

pub use self::sprite::{initialize_sprite_resource, SpriteResource};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EnemyEntityData {
    pub animation_component: AnimationComponent,
    pub enemy_component: EnemyComponent,
    pub hitbox_component: Hitbox2DComponent,
    pub blaster_component: Option<BlasterComponent>,
    pub autofire_component: Option<AutoFireComponent>,
    pub motion2d_component: Motion2DComponent,
    pub health_component: HealthComponent,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ThrusterEntityData {
    pub animation_component: AnimationComponent,
    pub y_offset: f32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ItemEntityData {
    pub item_component: ItemComponent,
    pub animation_component: Option<AnimationComponent>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConsumableEntityData {
    pub consumable_component: ConsumableComponent,
    pub hitbox_component: Hitbox2DComponent,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct DebugLinesConfig {
    pub line_width: f32,
    pub hitbox_color: Srgba,
}

pub type SoundsData = HashMap<String, String>;
pub type EnemyPool = HashMap<String, EnemyEntityData>;
pub type ThrusterPool = HashMap<String, ThrusterEntityData>;
pub type ItemPool = HashMap<String, ItemEntityData>;
pub type ConsumablePool = HashMap<String, ConsumableEntityData>;
