use crate::components::{Animation, Enemy, Hitbox2DComponent};
use amethyst::{
    assets::Handle,
    core::{math::Vector3, transform::Transform, Named},
    ecs::prelude::{Builder, Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, SpriteSheet, Transparent},
};

pub fn spawn_enemy(
    entities: &Entities,
    sprite_sheet: Handle<SpriteSheet>,
    enemy_component: Enemy,
    spawn_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) -> Entity {
    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: enemy_component.sprite_index,
    };

    // store animation data in seperate Animation component added to EnemyEntityData
    let animation = Animation {
        start_idx: enemy_component.sprite_index,
        frame_count: enemy_component.frame_count,
        current_frame: enemy_component.sprite_index,
        frame_time: enemy_component.frame_time,
        elapsed_time: 0.0,
        forward: true,
        animation_type: enemy_component.animation_type.clone(),
    };

    let name = Named::new("enemy");

    let hitbox = Hitbox2DComponent {
        width: 14.0,
        height: 14.0,
        offset_x: 0.0,
        offset_y: 0.0,
        offset_rotation: 0.0,
    };

    let mut local_transform = Transform::default();
    local_transform.set_translation(spawn_position);

    lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(enemy_component)
        .with(hitbox)
        .with(local_transform)
        .with(Transparent)
        .with(name)
        .with(animation)
        .build()

    /*
    super::spawn_animated_entity(
        &entities,
        name,
        sprite,
        animation,
        item,
        spawn_position,
        &lazy_update,
    )
    */
}
