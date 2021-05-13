use crate::{
    components::{
        AnimationComponent, AutoFireComponent, AutoSpawnerComponent, BlasterComponent,
        ConsumableComponent, DespawnAtBorderComponent, FadeComponent, HealthComponent,
        Hitbox2DComponent, ItemComponent, MobComponent, Motion2DComponent, TimeLimitComponent,
    },
    entities::{ConsumableType, EffectType, ItemType, MobType},
    resources::SpriteRenderData,
};

use amethyst::core::math::Vector2;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub random_initial_motion: RandomMotionRange2D,
    pub item_entities: HashMap<ItemType, ItemEntityData>,
}

pub type MobsResource = HashMap<MobType, MobEntityData>;
pub type EffectsResource = HashMap<EffectType, EffectEntityData>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MobEntityData {
    pub sprite_render_data: SpriteRenderData,
    pub animation_component: Option<AnimationComponent>,
    pub mob_component: MobComponent,
    pub hitbox_component: Hitbox2DComponent,
    pub blaster_component: Option<BlasterComponent>,
    pub autofire_component: Option<AutoFireComponent>,
    pub motion2d_component: Motion2DComponent,
    pub health_component: HealthComponent,
    pub despawn_component: DespawnAtBorderComponent,
    pub auto_spawner_component: Option<AutoSpawnerComponent>,
    pub thruster_data: Option<ThrusterEntityData>,
    pub random_initial_motion: Option<RandomMotionRange2D>,
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
    pub random_initial_motion: RandomMotionRange2D,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RandomMotionRange2D {
    pub linear: Option<Vector2<(f32, f32)>>,
    pub angular: Option<(f32, f32)>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EffectEntityData {
    pub random_initial_motion: Option<RandomMotionRange2D>,
    pub sprite_render_data: Vec<SpriteRenderData>,
    pub time_limit_component: Option<TimeLimitComponent>,
    pub motion2d_component: Option<Motion2DComponent>,
    pub animation_component: Option<AnimationComponent>,
    pub fade_component: Option<FadeComponent>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GibletEntityData {
    pub random_initial_motion: RandomMotionRange2D,
    pub time_limit_component: TimeLimitComponent,
    pub motion2d_component: Motion2DComponent,
    pub fade_component: FadeComponent,
}
