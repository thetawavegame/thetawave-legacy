//! Resources for spawnable entities
use crate::{
    motion::components::{Hitbox2DComponent, Motion2DComponent},
    spawn::components::DespawnAtBorderComponent,
    spawnable::components::BlastComponent,
    visual::resources::SpriteSheetsResource,
};

use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Builder, Entities, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent},
};

use serde::{Deserialize, Serialize};

mod modifiers;
mod spawnables;

pub use self::{
    modifiers::{ConsumableModifiersResource, ItemModifiersResource, Modifier},
    spawnables::{
        ConsumableEntityData, ConsumablesResource, EffectEntityData, EffectsResource,
        ItemEntityData, ItemsResource, MobEntityData, MobsResource, RandomMotionRange2D,
        ThrusterEntityData,
    },
};

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum SpawnableType {
    Consumable(ConsumableType),
    Item(ItemType),
    Effect(EffectType),
    Mob(MobType),
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum MobType {
    Enemy(EnemyType),
    Ally(AllyType),
    Neutral(NeutralType),
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum EnemyType {
    Pawn,
    Drone,
    StraferRight,
    StraferLeft,
    MissileLauncher,
    Missile,
    RepeaterBody,
    RepeaterHead,
    RepeaterLeftShoulder,
    RepeaterRightShoulder,
    RepeaterLeftArm,
    RepeaterRightArm,
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum AllyType {
    Hauler,
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum NeutralType {
    MoneyAsteroid,
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum ConsumableType {
    DefenseWrench,
    Money1,
    Money5,
    HealthWrench,
    Armor,
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum ItemType {
    SteelBarrel,
    PlasmaBlasts,
    HazardousReactor,
    WarpThruster,
    Tentaclover,
    DefenseSatellite,
    DoubleBarrel,
    YithianPlague,
    Spice,
    EnhancedPlating,
    StructureReinforcement,
    BlasterSizeEnhancer,
    FrequencyAugmentor,
    TractorBeam,
    BlastRepeller,
}
#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum EffectType {
    AllyBlastExplosion,
    EnemyBlastExplosion,
    PoisonBlastExplosion,
    CriticalBlastExplosion,
    MobExplosion,
    Star, //TODO: implement background stars
    Giblets(MobType),
}

/// Spawn any kind of spawnable entity
pub fn spawn_spawnable(
    spawnable_type: &SpawnableType,
    is_drop: bool,
    spawn_transform: Transform,
    consumables_resource: &ReadExpect<ConsumablesResource>,
    mobs_resource: &ReadExpect<MobsResource>,
    items_resource: &ReadExpect<ItemsResource>,
    effects_resource: &ReadExpect<EffectsResource>,
    spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    match spawnable_type {
        SpawnableType::Consumable(consumable_type) => {
            consumables_resource.spawn_consumable(
                consumable_type,
                is_drop,
                spawn_transform,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }

        SpawnableType::Mob(mob_type) => {
            mobs_resource.spawn_mob(
                mob_type,
                spawn_transform,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }

        SpawnableType::Item(item_type) => {
            items_resource.spawn_item(
                item_type,
                is_drop,
                spawn_transform,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }

        SpawnableType::Effect(effect_type) => {
            effects_resource.spawn_effect(
                effect_type,
                spawn_transform,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }
    }
}

/// Spawn blast entities
pub fn spawn_blasts(
    blast_count: usize,
    blast_spacing: f32,
    blast_sprite_render: SpriteRender,
    blast_component: BlastComponent,
    blast_hitbox: Hitbox2DComponent,
    blast_motion2d: Motion2DComponent,
    mut blast_transform: Transform,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    for _ in 0..blast_count {
        lazy_update
            .create_entity(entities)
            .with(blast_component.clone())
            .with(blast_hitbox.clone())
            .with(blast_motion2d.clone())
            .with(blast_sprite_render.clone())
            .with(blast_transform.clone())
            .with(Transparent)
            .with(DespawnAtBorderComponent {
                top_offset: Some(2.0),
                bottom_offset: Some(-2.0),
                left_offset: Some(-2.0),
                right_offset: Some(2.0),
            })
            .build();

        blast_transform.prepend_translation_x(blast_spacing);
    }
}
