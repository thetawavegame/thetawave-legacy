use crate::{
    components::{BarrierComponent, Hitbox2DComponent, PushDirection},
    constants::{
        ARENA_HEIGHT, ARENA_MAX_X, SIDE_PANEL_LEFT_SPRITE_INDEX, SIDE_PANEL_RIGHT_SPRITE_INDEX,
        SIDE_PANEL_WIDTH, SIDE_PANEL_Z,
    },
};
use amethyst::{
    assets::Handle,
    core::{math::Vector2, transform::Transform},
    ecs::{World, WorldExt},
    prelude::Builder,
    renderer::{SpriteRender, SpriteSheet},
};

pub fn initialize_arena_barriers(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite_render_left = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: SIDE_PANEL_LEFT_SPRITE_INDEX,
    };

    let sprite_render_right = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: SIDE_PANEL_RIGHT_SPRITE_INDEX,
    };

    let mut local_transform_left = Transform::default();
    local_transform_left.set_translation_xyz(
        SIDE_PANEL_WIDTH / 2.0,
        (ARENA_HEIGHT / 2.0) - 1.0,
        SIDE_PANEL_Z,
    );

    let mut local_transform_right = Transform::default();
    local_transform_right.set_translation_xyz(
        ARENA_MAX_X + (SIDE_PANEL_WIDTH / 2.0),
        (ARENA_HEIGHT / 2.0) - 1.0,
        SIDE_PANEL_Z,
    );

    let hitbox_component = Hitbox2DComponent {
        width: SIDE_PANEL_WIDTH,
        height: ARENA_HEIGHT,
        offset: Vector2::new(0.0, 0.0),
        offset_rotation: 0.0,
    };

    let left_barrier_component = BarrierComponent {
        deflection_speed: Vector2::new(30.0, 0.0),
        damage: 1.0,
        push_direction: PushDirection::Right,
    };

    let right_barrier_component = BarrierComponent {
        deflection_speed: Vector2::new(30.0, 0.0),
        damage: 1.0,
        push_direction: PushDirection::Left,
    };

    world
        .create_entity()
        .with(local_transform_left)
        .with(sprite_render_left)
        .with(hitbox_component.clone())
        .with(left_barrier_component)
        .build();

    world
        .create_entity()
        .with(local_transform_right)
        .with(sprite_render_right)
        .with(hitbox_component)
        .with(right_barrier_component)
        .build();
}
