use amethyst::{
    ecs::prelude::{Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent},
    core::{
        transform::Transform,
        math::Vector3,
    },
};
use crate::{
    components::Blast,
    resources::SpriteResource,
};

const HITBOX_RADIUS: f32 = 2.0;
const VELOCITY_FACTOR: f32 = 0.5; //amount that the spaceship/enemy's velocity affects the blast's velocity

pub fn fire_blast(entities: &Entities, blast_resource: &ReadExpect<SpriteResource>, sprite_number: usize, fire_position: Vector3<f32>, damage: f32, x_velocity: f32, y_velocity: f32, speed: f32, allied: bool, lazy_update: &ReadExpect<LazyUpdate>) {
    let blast_entity: Entity = entities.create();

    let mut local_transform = Transform::default();
    local_transform.set_translation(fire_position);

    let sprite_render = SpriteRender {
        sprite_sheet: blast_resource.blasts_sprite_sheet.clone(),
        sprite_number: sprite_number,
    };

    lazy_update.insert(blast_entity, sprite_render);
    lazy_update.insert(blast_entity, Blast {speed: speed, hitbox_radius: HITBOX_RADIUS, damage: damage, x_velocity: x_velocity, y_velocity: y_velocity, velocity_factor: VELOCITY_FACTOR, allied: allied});
    lazy_update.insert(blast_entity, local_transform);
    lazy_update.insert(blast_entity, Transparent);

}
