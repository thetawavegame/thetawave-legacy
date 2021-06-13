use crate::{
    components::{AnimationComponent, FadeComponent, HealthComponent},
    entities::{ConsumableType, EffectType, ItemType, MobType},
    motion::components::{Hitbox2DComponent, Motion2DComponent},
    resources::{SpriteRenderData, SpriteSheetsResource},
    spawn::components::{
        AutoSpawnerComponent, DespawnAtBorderComponent, DespawnTimeLimitComponent,
    },
    spawnable::components::{ConsumableComponent, ItemComponent, MobComponent},
    weapons::components::{AutoFireComponent, BlasterComponent},
};
use amethyst::{
    core::{math::Vector2, transform::Transform, Parent},
    ecs::prelude::{Builder, Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{palette::Srgba, resources::Tint, SpriteRender, Transparent},
};
use rand::{thread_rng, Rng};
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

impl ConsumablesResource {
    /// Spawn a consumable entity
    pub fn spawn_consumable(
        &self,
        consumable_type: &ConsumableType,
        is_drop: bool,
        spawn_transform: Transform,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        let consumable_data = &self.consumable_entities[consumable_type];

        let sprite_render = SpriteRender {
            sprite_sheet: spritesheets_resource.spritesheets
                [&consumable_data.sprite_render_data.spritesheet]
                .clone(),
            sprite_number: consumable_data.sprite_render_data.initial_index,
        };

        let mut motion2d_component = self.motion2d_component.clone();

        if is_drop {
            if let Some(linear_motion) = consumable_data.random_initial_motion.linear {
                motion2d_component.velocity.x =
                    thread_rng().gen_range(linear_motion.x.0, linear_motion.x.1);
                motion2d_component.velocity.y =
                    thread_rng().gen_range(linear_motion.y.0, linear_motion.y.1);
            }
        }

        if let Some(angular_motion) = consumable_data.random_initial_motion.angular {
            motion2d_component.angular_velocity =
                thread_rng().gen_range(angular_motion.0, angular_motion.1);
        }

        lazy_update
            .create_entity(entities)
            .with(sprite_render)
            .with(consumable_data.hitbox_component.clone())
            .with(consumable_data.consumable_component.clone())
            .with(motion2d_component)
            .with(spawn_transform)
            .with(Transparent)
            .with(self.despawn_border_component.clone())
            .build();
    }
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

impl ItemsResource {
    /// Spawn an item entity
    pub fn spawn_item(
        &self,
        item_type: &ItemType,
        is_drop: bool,
        spawn_transform: Transform,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        let item_data = &self.item_entities[item_type];

        let sprite_render = SpriteRender {
            sprite_sheet: spritesheets_resource.spritesheets
                [&item_data.sprite_render_data.spritesheet]
                .clone(),
            sprite_number: item_data.sprite_render_data.initial_index,
        };

        let mut motion2d_component = self.motion2d_component.clone();

        if is_drop {
            if let Some(linear_motion) = self.random_initial_motion.linear {
                motion2d_component.velocity.x =
                    thread_rng().gen_range(linear_motion.x.0, linear_motion.x.1);
                motion2d_component.velocity.y =
                    thread_rng().gen_range(linear_motion.y.0, linear_motion.y.1);
            }
        }
        if let Some(angular_motion) = self.random_initial_motion.angular {
            motion2d_component.angular_velocity =
                thread_rng().gen_range(angular_motion.0, angular_motion.1);
        }

        let item_entity = lazy_update
            .create_entity(entities)
            .with(sprite_render)
            .with(item_data.item_component.clone())
            .with(self.hitbox2d_component.clone())
            .with(motion2d_component)
            .with(spawn_transform)
            .with(Transparent)
            .with(self.despawn_border_component.clone())
            .build();

        if let Some(animation_component) = item_data.animation_component.clone() {
            lazy_update.insert(item_entity, animation_component);
        }
    }
}

/// Used for storing data for mob entities
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MobsResource {
    /// Mob types mapped to their unique mob data
    pub mob_entities: HashMap<MobType, MobEntityData>,
}

impl MobsResource {
    /// Spawn a mob entity
    pub fn spawn_mob(
        &self,
        mob_type: &MobType,
        spawn_transform: Transform,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) -> Entity {
        let mob_data = &self.mob_entities[mob_type];

        let mob_sprite_render = SpriteRender {
            sprite_sheet: spritesheets_resource.spritesheets
                [&mob_data.sprite_render_data.spritesheet]
                .clone(),
            sprite_number: mob_data.sprite_render_data.initial_index,
        };

        let mut motion2d_component = mob_data.motion2d_component.clone();

        if let Some(random_initial_motion) = mob_data.random_initial_motion.clone() {
            if let Some(linear_motion) = random_initial_motion.linear {
                motion2d_component.velocity.x =
                    thread_rng().gen_range(linear_motion.x.0, linear_motion.x.1);
                motion2d_component.velocity.y =
                    thread_rng().gen_range(linear_motion.y.0, linear_motion.y.1);
            }

            if let Some(angular_motion) = random_initial_motion.angular {
                motion2d_component.angular_velocity =
                    thread_rng().gen_range(angular_motion.0, angular_motion.1);
            }
        }

        let mob_entity = lazy_update
            .create_entity(entities)
            .with(mob_sprite_render)
            .with(mob_data.mob_component.clone())
            .with(mob_data.hitbox_component.clone())
            .with(motion2d_component)
            .with(mob_data.health_component.clone())
            .with(mob_data.despawn_component.clone())
            .with(spawn_transform)
            .with(Transparent)
            .build();

        if let Some(animation_component) = mob_data.animation_component.clone() {
            lazy_update.insert(mob_entity, animation_component);
        }
        if let Some(blaster_component) = mob_data.blaster_component.clone() {
            lazy_update.insert(mob_entity, blaster_component);
        }
        if let Some(autofire_component) = mob_data.autofire_component.clone() {
            lazy_update.insert(mob_entity, autofire_component);
        }
        if let Some(auto_child_entity_spawner_component) = mob_data.auto_spawner_component.clone() {
            lazy_update.insert(mob_entity, auto_child_entity_spawner_component);
        }

        // spawn thruster entity as child of mob entity
        if let Some(thruster_data) = mob_data.thruster_data.clone() {
            let thruster_parent = Parent::new(mob_entity);

            let thruster_sprite_render = SpriteRender {
                sprite_sheet: spritesheets_resource.spritesheets["thrusters"].clone(),
                sprite_number: thruster_data.animation_component.start_idx,
            };

            let mut thruster_transform = Transform::default();
            thruster_transform.set_translation_y(thruster_data.y_offset);

            lazy_update
                .create_entity(entities)
                .with(thruster_parent)
                .with(thruster_transform)
                .with(thruster_sprite_render)
                .with(thruster_data.animation_component)
                .with(Transparent)
                .build();
        }

        mob_entity
    }
}

/// Used for storing data for effect entities
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EffectsResource {
    /// Effect types mapped to their unique effect data
    pub effect_entities: HashMap<EffectType, EffectEntityData>,
}

impl EffectsResource {
    /// Spawn effect entities
    pub fn spawn_effect(
        &self,
        effect_type: &EffectType,
        spawn_transform: Transform,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        let effect_data = &self.effect_entities[effect_type];

        for sprite_render_data in effect_data.sprite_render_data.iter() {
            // for each spriterender data in the array create an entity with a unique sprite render and the same other properties

            let sprite_render = SpriteRender {
                sprite_sheet: spritesheets_resource.spritesheets[&sprite_render_data.spritesheet]
                    .clone(),
                sprite_number: sprite_render_data.initial_index,
            };

            let effect_entity = lazy_update
                .create_entity(entities)
                .with(sprite_render)
                .with(spawn_transform.clone())
                .with(Transparent)
                .build();

            // TODO: scale to be a vector (like with Sprite render)
            if let Some(animation_component) = effect_data.animation_component.clone() {
                lazy_update.insert(effect_entity, animation_component);
            }

            if let Some(time_limit_component) = effect_data.time_limit_component.clone() {
                lazy_update.insert(effect_entity, time_limit_component);
            }

            if let Some(mut motion2d_component) = effect_data.motion2d_component.clone() {
                if let Some(random_initial_motion) = effect_data.random_initial_motion.clone() {
                    if let Some(linear_motion) = random_initial_motion.linear {
                        motion2d_component.velocity.x =
                            thread_rng().gen_range(linear_motion.x.0, linear_motion.x.1);
                        motion2d_component.velocity.y =
                            thread_rng().gen_range(linear_motion.y.0, linear_motion.y.1);
                    }

                    if let Some(angular_motion) = random_initial_motion.angular {
                        motion2d_component.angular_velocity =
                            thread_rng().gen_range(angular_motion.0, angular_motion.1);
                    }
                }
                lazy_update.insert(effect_entity, motion2d_component);
            } else if effect_data.random_initial_motion.is_some() {
                panic!("Effects with RandomInitialMotion must also have a motion component.")
            }

            if let Some(fade_component) = effect_data.fade_component.clone() {
                let tint_component = Tint(Srgba::new(
                    if let Some(red_change) = fade_component.red_change.clone() {
                        red_change.value
                    } else {
                        1.0
                    },
                    if let Some(green_change) = fade_component.green_change.clone() {
                        green_change.value
                    } else {
                        1.0
                    },
                    if let Some(blue_change) = fade_component.blue_change.clone() {
                        blue_change.value
                    } else {
                        1.0
                    },
                    if let Some(alpha_change) = fade_component.alpha_change.clone() {
                        alpha_change.value
                    } else {
                        1.0
                    },
                ));

                lazy_update.insert(effect_entity, tint_component);
                lazy_update.insert(effect_entity, fade_component);
            }
        }
    }
}

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
