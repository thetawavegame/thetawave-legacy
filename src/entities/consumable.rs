use crate::{
    components::{
        choose_random_name, ConsumableComponent, DespawnAtBorderComponent, EnemyComponent,
    },
    resources::{ConsumableEntityData, ConsumablesResource, SpriteSheetsResource},
};
use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{Builder, Entities, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent},
};

pub fn spawn_consumable(
    entities: &Entities,
    sprite_resource: &ReadExpect<SpriteSheetsResource>,
    consumable_data: &ConsumableEntityData,
    consumables_resource: &ReadExpect<ConsumablesResource>,
    spawn_position: &Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_resource.spritesheets["consumables"].clone(),
        sprite_number: consumable_data.consumable_component.sprite_index,
    };

    let mut local_transform = Transform::default();
    local_transform.set_translation(*spawn_position);

    lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(consumable_data.hitbox_component.clone())
        .with(consumable_data.consumable_component.clone())
        .with(consumables_resource.motion2d_component.clone())
        .with(local_transform)
        .with(Transparent)
        .with(DespawnAtBorderComponent {
            top_offset: None,
            bottom_offset: Some(20.0),
            left_offset: None,
            right_offset: None,
        })
        .build();
}

pub fn spawn_random_consumable(
    entities: &Entities,
    enemy: &EnemyComponent,
    sprite_resource: &ReadExpect<SpriteSheetsResource>,
    consumables_resource: &ReadExpect<ConsumablesResource>,
    spawn_position: &Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let name = choose_random_name(&enemy.collectables_probs);

    if !name.is_empty() {
        spawn_consumable(
            entities,
            sprite_resource,
            &consumables_resource.consumable_entities[name],
            consumables_resource,
            spawn_position,
            lazy_update,
        )
    }
}
