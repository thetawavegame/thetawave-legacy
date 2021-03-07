use crate::{
    components::{choose_random_entity, EnemyComponent},
    entities::{spawn_consumable, EntityType},
    resources::{ConsumableEntityData, ConsumablesResource, SpriteSheetsResource},
};
use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{Builder, Entities, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent},
};

pub fn spawn_random_consumable(
    entities: &Entities,
    enemy: &EnemyComponent,
    sprite_resource: &ReadExpect<SpriteSheetsResource>,
    consumables_resource: &ReadExpect<ConsumablesResource>,
    spawn_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let consumable_type = choose_random_entity(&enemy.loot_probs);

    if let Some(EntityType::Consumable(consumable_type)) = consumable_type {
        spawn_consumable(
            consumable_type,
            spawn_position,
            consumables_resource,
            sprite_resource,
            entities,
            lazy_update,
        )
    }
}
