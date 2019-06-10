use amethyst::{
    ecs::prelude::{Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{SpriteRender},
    core::{
        transform::Transform,
        nalgebra::Vector3,
    },
};

use crate::{
    resources::SpriteResource,
};


pub fn spawn_health_unit(entities: &Entities, sprite_resource: &ReadExpect<SpriteResource>, sprite_number: usize, spawn_position: Vector3<f32>, lazy_update: &ReadExpect<LazyUpdate>) -> Entity {
    let health_unit_entitiy: Entity = entities.create();

    let mut local_transform = Transform::default();
    local_transform.set_position(spawn_position);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_resource.sprite_sheet.clone(),
        sprite_number: sprite_number,
    };


    lazy_update.insert(health_unit_entitiy, sprite_render);
    lazy_update.insert(health_unit_entitiy, local_transform);

    health_unit_entitiy

}