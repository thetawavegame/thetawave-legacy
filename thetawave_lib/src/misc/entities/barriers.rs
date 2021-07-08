use crate::{
    constants::{
        ARENA_HEIGHT, ARENA_MAX_X, ARENA_MAX_Y, ARENA_MIN_Y, ARENA_WIDTH, SIDE_PANEL_WIDTH,
        SIDE_PANEL_Z,
    },
    misc::components::{BarrierComponent, PushDirection},
    motion::Hitbox2DComponent,
};
use amethyst::{
    core::{math::Vector2, transform::Transform},
    ecs::{World, WorldExt},
    prelude::Builder,
};

pub fn initialize_arena_barriers(world: &mut World) {
    let mut local_transform_left = Transform::default();
    local_transform_left.set_translation_xyz(
        SIDE_PANEL_WIDTH - 10.0,
        (ARENA_HEIGHT / 2.0) - 1.0,
        SIDE_PANEL_Z,
    );

    let mut local_transform_right = Transform::default();
    local_transform_right.set_translation_xyz(
        ARENA_MAX_X + 10.0,
        (ARENA_HEIGHT / 2.0) - 1.0,
        SIDE_PANEL_Z,
    );

    let mut local_transform_top = Transform::default();
    local_transform_top.set_translation_xyz(
        SIDE_PANEL_WIDTH + (ARENA_WIDTH / 2.0),
        ARENA_MAX_Y + 10.0,
        SIDE_PANEL_Z,
    );

    let mut local_transform_bottom = Transform::default();
    local_transform_bottom.set_translation_xyz(
        SIDE_PANEL_WIDTH + (ARENA_WIDTH / 2.0),
        ARENA_MIN_Y - 10.0,
        SIDE_PANEL_Z,
    );

    let vertical_hitbox_component = Hitbox2DComponent {
        width: 20.0,
        height: ARENA_HEIGHT,
        offset: Vector2::new(0.0, 0.0),
        offset_rotation: 0.0,
    };

    let horizontal_hitbox_component = Hitbox2DComponent {
        width: ARENA_WIDTH,
        height: 20.0,
        offset: Vector2::new(0.0, 0.0),
        offset_rotation: 0.0,
    };

    let left_barrier_component = BarrierComponent {
        deflection_speed: Vector2::new(30.0, 0.0),
        damage: 1.0,
        enemies_pass: false,
        push_direction: PushDirection::Right,
    };

    let right_barrier_component = BarrierComponent {
        deflection_speed: Vector2::new(30.0, 0.0),
        damage: 1.0,
        enemies_pass: false,
        push_direction: PushDirection::Left,
    };

    let top_barrier_component = BarrierComponent {
        deflection_speed: Vector2::new(0.0, 30.0),
        damage: 1.0,
        enemies_pass: true,
        push_direction: PushDirection::Down,
    };

    let bottom_barrier_component = BarrierComponent {
        deflection_speed: Vector2::new(0.0, 30.0),
        damage: 1.0,
        enemies_pass: true,
        push_direction: PushDirection::Up,
    };

    world
        .create_entity()
        .with(local_transform_left)
        .with(vertical_hitbox_component.clone())
        .with(left_barrier_component)
        .build();

    world
        .create_entity()
        .with(local_transform_right)
        .with(vertical_hitbox_component)
        .with(right_barrier_component)
        .build();

    world
        .create_entity()
        .with(local_transform_top)
        .with(horizontal_hitbox_component.clone())
        .with(top_barrier_component)
        .build();

    world
        .create_entity()
        .with(local_transform_bottom)
        .with(horizontal_hitbox_component)
        .with(bottom_barrier_component)
        .build();
}
