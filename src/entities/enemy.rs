use crate::{components::Animation, resources::EnemyEntityData};
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
        sprite_number: enemy.enemy_component.sprite_index,
    };

    // store animation data in separate Animation component added to EnemyEntityData
    let animation = Animation {
        start_idx: enemy.enemy_component.sprite_index,
        frame_count: enemy.enemy_component.frame_count,
        current_frame: enemy.enemy_component.sprite_index,
        frame_time: enemy.enemy_component.frame_time,
        elapsed_time: 0.0,
        forward: true,
        animation_type: enemy.enemy_component.animation_type.clone(),
    };

    let name = Named::new("enemy");

    let mut local_transform = Transform::default();
    local_transform.set_translation(spawn_position);

    lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(enemy.enemy_component)
        .with(enemy.hitbox_component)
        .with(local_transform)
        .with(Transparent)
        .with(name)
        .with(animation)
        .build()
}
