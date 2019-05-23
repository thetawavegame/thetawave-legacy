use amethyst::{
    ecs::prelude::{World, Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{SpriteSheetHandle, SpriteRender, Transparent},
    core::{
        transform::Transform,
        nalgebra::Vector3,
    },
};

use crate::{
    resources::ExplosionResource,
    components::Explosion,
};

const EXPLOSION_DURATION: f32 = 0.3;

pub fn initialise_explosion_resource(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) -> ExplosionResource {
    let explosion_resource = ExplosionResource {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 3,
    };

    world.add_resource(explosion_resource.clone());
    explosion_resource
}

pub fn spawn_explosion(entities: &Entities, explosion_resource: &ReadExpect<ExplosionResource>, spawn_position: Vector3<f32>, lazy_update: &ReadExpect<LazyUpdate>) {
    let explosion_entitiy: Entity = entities.create();

    let mut local_transform = Transform::default();
    local_transform.set_position(spawn_position);

    let sprite_render = SpriteRender {
        sprite_sheet: explosion_resource.sprite_sheet.clone(),
        sprite_number: explosion_resource.sprite_number,
    };


    lazy_update.insert(explosion_entitiy, sprite_render);
    lazy_update.insert(explosion_entitiy, Explosion{duration: EXPLOSION_DURATION});
    lazy_update.insert(explosion_entitiy, local_transform);
    lazy_update.insert(explosion_entitiy, Transparent);

}
