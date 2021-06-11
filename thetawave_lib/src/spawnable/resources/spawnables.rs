use crate::{
    components::{
        AnimationComponent, FadeComponent, HealthComponent, Hitbox2DComponent, Motion2DComponent,
    },
    entities::{ConsumableType, EffectType, ItemType, MobType},
    resources::SpriteRenderData,
    spawn::components::{
        AutoSpawnerComponent, DespawnAtBorderComponent, DespawnTimeLimitComponent,
    },
    spawnable::components::{ConsumableComponent, ItemComponent, MobComponent},
    weapons::components::{AutoFireComponent, BlasterComponent},
};
use amethyst::core::math::Vector2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Used for storing data for consumable entities
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConsumablesResource {
    /// Motion2D component shared with all entities
    pub motion2d_component: Motion2DComponent,
    /// Despawn at border component shared with all entities
    pub despawn_border_component: DespawnAtBorderComponent,
    /// Consumable types mapped to their unique consumable data
    pub consumable_entities: HashMap<ConsumableType, ConsumableEntityData>,
}

/// Used for storing data for item entities
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ItemsResource {
    /// Motion2D component shared with all entities
    pub motion2d_component: Motion2DComponent,
    /// Hitbox2D component shared with all entities
    pub hitbox2d_component: Hitbox2DComponent,
    /// Despawn at border component shared with all entities
    pub despawn_border_component: DespawnAtBorderComponent,
    /// Random initial motion shared with all entities
    pub random_initial_motion: RandomMotionRange2D,
    /// Item types mapped to their unique item data
    pub item_entities: HashMap<ItemType, ItemEntityData>,
}

/// Mob types mapped to their unique data
pub type MobsResource = HashMap<MobType, MobEntityData>;
/// Effect types mapped to their unique data
pub type EffectsResource = HashMap<EffectType, EffectEntityData>;

/// Unique mob entity data
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MobEntityData {
    /// Sprite data
    pub sprite_render_data: SpriteRenderData,
    /// Optional animation component
    pub animation_component: Option<AnimationComponent>,
    /// Mob component
    pub mob_component: MobComponent,
    /// Hitbox2D component
    pub hitbox_component: Hitbox2DComponent,
    /// Optional blaster component
    pub blaster_component: Option<BlasterComponent>,
    /// Optional auto-fire component
    pub autofire_component: Option<AutoFireComponent>,
    /// Motion2D component
    pub motion2d_component: Motion2DComponent,
    /// Health component
    pub health_component: HealthComponent,
    /// Despawn at border component
    pub despawn_component: DespawnAtBorderComponent,
    /// Optional auto-spawner component
    pub auto_spawner_component: Option<AutoSpawnerComponent>,
    /// Optional thruster entity data
    pub thruster_data: Option<ThrusterEntityData>,
    /// Optional random 2D motion range
    pub random_initial_motion: Option<RandomMotionRange2D>,
}

/// Unique thruster entity data
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ThrusterEntityData {
    /// Animation component
    pub animation_component: AnimationComponent,
    /// Y offset from the position of the parent entity
    pub y_offset: f32,
}

/// Unique item entity data
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ItemEntityData {
    /// Sprite data
    pub sprite_render_data: SpriteRenderData,
    /// Item component
    pub item_component: ItemComponent,
    /// Optional animation component
    pub animation_component: Option<AnimationComponent>,
}

/// Unique consumable entity data
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConsumableEntityData {
    /// Sprite data
    pub sprite_render_data: SpriteRenderData,
    /// Consumable component
    pub consumable_component: ConsumableComponent,
    /// Hitbox2D component
    pub hitbox_component: Hitbox2DComponent,
    /// Random 2D motion range
    pub random_initial_motion: RandomMotionRange2D,
}

/// Initial motion random motion
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RandomMotionRange2D {
    /// Optional linear motion range
    pub linear: Option<Vector2<(f32, f32)>>,
    /// Optional angular motion range
    pub angular: Option<(f32, f32)>,
}

/// Unique effect entity data
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EffectEntityData {
    /// Optional random 2D motion range
    pub random_initial_motion: Option<RandomMotionRange2D>,
    /// Vector of sprite data
    pub sprite_render_data: Vec<SpriteRenderData>,
    /// Optional time limit component
    pub time_limit_component: Option<DespawnTimeLimitComponent>,
    /// Optional motion2d component
    pub motion2d_component: Option<Motion2DComponent>,
    /// Optional animation component
    pub animation_component: Option<AnimationComponent>,
    /// Optional fade component
    pub fade_component: Option<FadeComponent>,
}
