use crate::{
    components::{
        AnimationComponent, AutoChildEntitySpawnerComponent, AutoFireComponent, BlasterComponent,
        ConsumableComponent, DespawnAtBorderComponent, EnemyComponent, HealthComponent,
        Hitbox2DComponent, ItemComponent, Motion2DComponent, TimeLimitComponent,
    },
    entities::{ConsumableType, EffectType, EnemyType, ItemType},
    resources::SpriteRenderData,
};

use amethyst::core::math::Vector2;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type RandomRange2D = Vector2<(f32, f32)>;

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
    pub item_entities: HashMap<ItemType, ItemEntityData>,
}

pub type EnemiesResource = HashMap<EnemyType, EnemyEntityData>;
pub type EffectsResource = HashMap<EffectType, EffectEntityData>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EnemyEntityData {
    pub sprite_render_data: SpriteRenderData,
    pub animation_component: AnimationComponent,
    pub enemy_component: EnemyComponent,
    pub hitbox_component: Hitbox2DComponent,
    pub blaster_component: Option<BlasterComponent>,
    pub autofire_component: Option<AutoFireComponent>,
    pub motion2d_component: Motion2DComponent,
    pub health_component: HealthComponent,
    pub despawn_component: DespawnAtBorderComponent,
    pub auto_child_entity_spawner_component: Option<AutoChildEntitySpawnerComponent>,
    pub thruster_data: Option<ThrusterEntityData>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ThrusterEntityData {
    pub animation_component: AnimationComponent,
    pub y_offset: f32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ItemEntityData {
    pub sprite_render_data: SpriteRenderData,
    pub item_component: ItemComponent,
    pub animation_component: Option<AnimationComponent>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConsumableEntityData {
    pub sprite_render_data: SpriteRenderData,
    pub consumable_component: ConsumableComponent,
    pub hitbox_component: Hitbox2DComponent,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RandomInitialMotion {
    pub initial_base: Vector2<f32>,
    pub random_offset: RandomRange2D,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EffectEntityData {
    pub random_initial_motion: Option<RandomInitialMotion>,
    pub sprite_render_data: SpriteRenderData,
    pub time_limit_component: Option<TimeLimitComponent>,
    pub motion2d_component: Option<Motion2DComponent>,
    pub animation_component: Option<AnimationComponent>,
}
