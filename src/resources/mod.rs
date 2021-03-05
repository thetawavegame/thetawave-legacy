use crate::{
    components::{
        AnimationComponent, AutoChildEnemySpawnerComponent, AutoFireComponent, BlasterComponent,
        ConsumableComponent, DespawnAtBorderComponent, EnemyComponent, HealthComponent,
        Hitbox2DComponent, ItemComponent, Motion2DComponent, PlayerComponent,
    },
    entities::{ConsumableType, EnemyType, ItemType},
};
use amethyst::{
    assets::Handle,
    renderer::{palette::Srgba, SpriteSheet},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod game_parameters;
pub mod phases;

pub use self::game_parameters::GameParametersResource;
pub use self::phases::{BossType, Phase, PhaseManagerResource, PhaseType};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EnemyEntityData {
    pub animation_component: AnimationComponent,
    pub enemy_component: EnemyComponent,
    pub hitbox_component: Hitbox2DComponent,
    pub blaster_component: Option<BlasterComponent>,
    pub autofire_component: Option<AutoFireComponent>,
    pub motion2d_component: Motion2DComponent,
    pub health_component: HealthComponent,
    pub despawn_component: DespawnAtBorderComponent,
    pub auto_child_enemy_spawner_component: Option<AutoChildEnemySpawnerComponent>,
    pub thruster_data: Option<ThrusterEntityData>,
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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PlayerEntityData {
    pub player_component: PlayerComponent,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SpriteSheetData {
    pub image: String,
    pub data: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConsumablesResource {
    pub motion2d_component: Motion2DComponent,
    pub despawn_border_component: DespawnAtBorderComponent,
    pub consumable_entities: HashMap<ConsumableType, ConsumableEntityData>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ItemsResource {
    pub motion2d_component: Motion2DComponent,
    pub hitbox2d_component: Hitbox2DComponent,
    pub despawn_border_component: DespawnAtBorderComponent,
    pub item_entities: HashMap<String, ItemEntityData>,
}

pub type EnemiesResource = HashMap<EnemyType, EnemyEntityData>;
//pub type ThrustersResource = HashMap<EnemyType, ThrusterEntityData>;
pub type PlayersResource = HashMap<String, PlayerEntityData>;

#[derive(Clone)]
pub struct SpriteSheetsResource {
    pub spritesheets: HashMap<String, Handle<SpriteSheet>>,
}

pub type SpriteSheetsConfig = HashMap<String, SpriteSheetData>;
pub type SoundsConfig = HashMap<String, String>;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct DebugLinesConfig {
    pub line_width: f32,
    pub hitbox_color: Srgba,
}
