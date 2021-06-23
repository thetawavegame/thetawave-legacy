use crate::resources::SpriteSheetsResource;
use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{Entities, Entity, LazyUpdate, ReadExpect},
    renderer::SpriteRender,
};

/// Spawn a status unit for a status bar
pub fn spawn_status_unit(
    entities: &Entities,
    sprite_resource: &ReadExpect<SpriteSheetsResource>,
    sprite_number: usize,
    spawn_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) -> Entity {
    let status_unit_entity: Entity = entities.create();

    let mut local_transform = Transform::default();
    local_transform.set_translation(spawn_position);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_resource.spritesheets["status_bar_unit"].clone(),
        sprite_number,
    };

    lazy_update.insert(status_unit_entity, sprite_render);
    lazy_update.insert(status_unit_entity, local_transform);

    status_unit_entity
}
