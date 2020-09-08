use crate::resources::EnemyEntityData;
use amethyst::{
    assets::Handle,
    core::{math::Vector3, transform::Transform, Named},
    ecs::prelude::{Builder, Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, SpriteSheet, Transparent},
};

pub fn spawn_enemy(
    entities: &Entities,
    sprite_sheet: Handle<SpriteSheet>,
    enemy: EnemyEntityData,
    spawn_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) -> Entity {
    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: enemy.animation_component.start_idx,
    };

    let name = Named::new("enemy");

    let mut local_transform = Transform::default();
    local_transform.set_translation(spawn_position);

    lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(enemy.animation_component)
        .with(enemy.enemy_component)
        .with(enemy.hitbox_component)
        .with(enemy.motion2d_component)
        .with(local_transform)
        .with(Transparent)
        .with(name)
        .build()
}
