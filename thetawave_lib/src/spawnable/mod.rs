//! `thetawave_lib` spawnable module

mod components;
mod mob_systems;
mod modifier_resources;
mod modifiers_system;
mod spawnable_resources;
mod spawnable_types;

pub use self::{
    components::{BlastComponent, ConsumableComponent, ItemComponent, MobComponent},
    mob_systems::{MobBehaviorSystem, MobDestroyedSystem},
    modifier_resources::{ConsumableModifiersResource, ItemModifiersResource, Modifier},
    modifiers_system::ModifiersSystem,
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
