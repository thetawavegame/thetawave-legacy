//! `thetawave_lib` spawnable module

pub mod components;
mod modifier_resources;
mod spawnable_resources;
mod spawnable_types;
pub mod systems;

pub use self::{
    modifier_resources::{ConsumableModifiersResource, ItemModifiersResource, Modifier},
    spawnable_resources::{
        spawn_blasts, spawn_spawnable, ConsumableEntityData, ConsumablesResource, EffectEntityData,
        EffectsResource, ItemEntityData, ItemsResource, MobEntityData, MobsResource,
        SpawnableResources,
    },
    spawnable_types::{
        AllyType, ConsumableType, EffectType, EnemyType, ItemType, MobType, NeutralType,
        SpawnableType,
    },
};
