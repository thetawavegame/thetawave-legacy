use crate::{
    components::{
        AbilityDirection, AnimationComponent, AnimationType, BarrelRollAbilityComponent, BlastType,
        BlasterComponent, HealthComponent, Hitbox2DComponent, ManualFireComponent,
        Motion2DComponent, OpaqueFadeComponent,
    },
    constants::{
        ARENA_HEIGHT, ARENA_MIN_X, ARENA_MIN_Y, ARENA_WIDTH, CRIT_BLAST_SPRITE_INDEX,
        POISON_BLAST_SPRITE_INDEX, SPACESHIP_ACCELERATION_X, SPACESHIP_ACCELERATION_Y,
        SPACESHIP_BARREL_COOLDOWN, SPACESHIP_BARREL_DURATION, SPACESHIP_BARREL_SPEED,
        SPACESHIP_BLAST_SPRITE_INDEX, SPACESHIP_DAMAGE, SPACESHIP_DECELERATION_X,
        SPACESHIP_DECELERATION_Y, SPACESHIP_FIRE_SPEED, SPACESHIP_HEALTH, SPACESHIP_HITBOX_HEIGHT,
        SPACESHIP_HITBOX_WIDTH, SPACESHIP_MAX_KNOCKBACK_SPEED, SPACESHIP_MAX_SPEED,
    },
    resources::PlayersResource,
};
use amethyst::{
    assets::Handle,
    core::{
        math::{Vector2, Vector3},
        transform::Transform,
    },
    ecs::{World, WorldExt},
    prelude::Builder,
    renderer::{palette::Srgba, resources::Tint, SpriteRender, SpriteSheet, Transparent},
};
use std::collections::HashMap;

pub fn initialize_background(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(23500.0, 135.0, -15000.0);
    local_transform.set_scale(Vector3::new(65.0, 65.0, 0.0));

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    let motion_2d = Motion2DComponent {
        velocity: Vector2::new(-120.0, 0.0),
        acceleration: Vector2::new(0.0, 0.0),
        deceleration: Vector2::new(0.0, 0.0),
        angular_velocity: 0.0,
        angular_acceleration: 0.0,
        angular_deceleration: 0.0,
        angular_speed: 0.0,
        speed: Vector2::new(120.0, 0.0),
        max_speed: Vector2::new(120.0, 0.0),
        immovable: false,
    };

    let animation = AnimationComponent {
        start_idx: 0,
        frame_count: 6,
        current_frame: 0,
        frame_time: 0.2,
        elapsed_time: 0.0,
        forward: true,
        animation_type: AnimationType::PingPong,
    };

    let current_color_value = 0.05;

    let opaque_fade = OpaqueFadeComponent {
        color_change: 0.00005,
        max_color_value: 0.58,
        min_color_value: 0.0,
        current_color_value,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(motion_2d)
        .with(local_transform)
        .with(animation)
        .with(Tint(Srgba::new(
            current_color_value,
            current_color_value,
            current_color_value,
            1.0,
        )))
        .with(opaque_fade)
        .build();
}
