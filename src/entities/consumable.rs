use crate::{
    components::DespawnAtBorderComponent,
    resources::{ConsumablesResource, SpriteSheetsResource},
};
use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{Builder, Entities, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent},
};

pub fn spawn_consumable(
    entities: &Entities,
    sprite_resource: &ReadExpect<SpriteSheetsResource>,
    consumables_resource: &ReadExpect<ConsumablesResource>,
    name: String,
    spawn_position: &Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_resource.spritesheets["consumables"].clone(),
        sprite_number: consumables_resource.consumable_entities[&name]
            .consumable_component
            .sprite_index,
    };

    let mut local_transform = Transform::default();
    local_transform.set_translation(*spawn_position);

    lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(
            consumables_resource.consumable_entities[&name]
                .hitbox_component
                .clone(),
        )
        .with(
            consumables_resource.consumable_entities[&name]
                .consumable_component
                .clone(),
        )
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
