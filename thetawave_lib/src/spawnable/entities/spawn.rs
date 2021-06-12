use crate::{
    entities::SpawnableType,
    resources::SpriteSheetsResource,
    spawnable::resources::{ConsumablesResource, EffectsResource, ItemsResource, MobsResource},
};
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities, LazyUpdate, ReadExpect},
};

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
