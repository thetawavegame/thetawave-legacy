use crate::{
    components::{choose_random_entity, MobComponent},
    entities::{spawn_consumable, SpawnableType},
    resources::{ConsumablesResource, SpriteSheetsResource},
};
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities, LazyUpdate, ReadExpect},
};

pub fn spawn_random_consumable(
    entities: &Entities,
    mob: &MobComponent,
    spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
    consumables_resource: &ReadExpect<ConsumablesResource>,
    spawn_transform: Transform,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let consumable_type = choose_random_entity(&mob.loot_probs);

    if let Some(SpawnableType::Consumable(consumable_type)) = consumable_type {
        spawn_consumable(
            consumable_type,
            spawn_transform,
            consumables_resource,
            spritesheets_resource,
            entities,
            lazy_update,
        )
    }
}
