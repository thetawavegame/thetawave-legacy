use amethyst::{
    ecs::prelude::{Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent},
    core::{
        transform::Transform,
        nalgebra::Vector3,
    },
};

use crate::{
    resources::SpriteResource,
    components::Explosion,
};


const EXPLOSION_DURATION: f32 = 0.3;


pub fn spawn_explosion(entities: &Entities, explosion_resource: &ReadExpect<SpriteResource>, sprite_number: usize, spawn_position: Vector3<f32>, lazy_update: &ReadExpect<LazyUpdate>) {
    let explosion_entitiy: Entity = entities.create();

    let mut local_transform = Transform::default();
    local_transform.set_position(spawn_position);

    let sprite_render = SpriteRender {
        sprite_sheet: explosion_resource.sprite_sheet.clone(),
        sprite_number: sprite_number,
    };

    lazy_update.insert(explosion_entitiy, sprite_render);
    lazy_update.insert(explosion_entitiy, Explosion{duration: EXPLOSION_DURATION});
    lazy_update.insert(explosion_entitiy, local_transform);
    lazy_update.insert(explosion_entitiy, Transparent);

}
