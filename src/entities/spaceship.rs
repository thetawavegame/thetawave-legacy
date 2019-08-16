use amethyst::{
    prelude::Builder,
    ecs::prelude::World,
    core::transform::Transform,
    renderer::{SpriteRender, SpriteSheet, Transparent},
    assets::Handle,
};

use crate::{
    components::Spaceship,
    space_shooter::{ARENA_MIN_X, ARENA_MIN_Y, ARENA_WIDTH, ARENA_HEIGHT},
};


const HEIGHT: f32 = 18.0;
const WIDTH: f32 = 18.0;
const HITBOX_HEIGHT: f32 = 14.0;
const HITBOX_WIDTH: f32 = 6.0;
const ACCELERATION_X: f32 = 2.0;
const DECELERATION_X: f32 = 1.0;
const ACCELERATION_Y: f32 = 4.0;
const DECELERATION_Y: f32 = 1.0;
const MAX_SPEED: f32 = 70.0;
const MAX_KNOCKBACK_SPEED: f32 = 100.0;
const FIRE_SPEED: f32 = 0.3;
const DAMAGE: f32 = 40.0;
const BARREL_COOLDOWN: f32 = 1.0;
const BARREL_SPEED: f32 = 180.0;
const BARREL_DURATION: f32 = 0.3;
const HEALTH: f32 = 400.0;
const COLLISION_DAMAGE: f32 = 50.0;


pub fn initialise_spaceship(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {

    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_MIN_X + (ARENA_WIDTH / 2.0), ARENA_MIN_Y + (ARENA_HEIGHT / 6.0), 0.9);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Spaceship {
            width: WIDTH,
            height: HEIGHT,
            hitbox_width: HITBOX_WIDTH,
            hitbox_height: HITBOX_HEIGHT,
            max_speed: MAX_SPEED,
            current_velocity_x: 0.0,
            current_velocity_y: 0.0,
            acceleration_x: ACCELERATION_X,
            deceleration_x: DECELERATION_X,
            acceleration_y: ACCELERATION_Y,
            deceleration_y: DECELERATION_Y,
            fire_speed: FIRE_SPEED,
            fire_reset_timer: 0.0,
            damage: DAMAGE,
            barrel_cooldown: BARREL_COOLDOWN,
            barrel_reset_timer: 0.0,
            barrel_speed: BARREL_SPEED,
            barrel_action_right: false,
            barrel_action_left: false,
            barrel_duration: BARREL_DURATION,
            barrel_action_timer: BARREL_DURATION,
            pos_x: local_transform.translation().x,
            pos_y: local_transform.translation().y,
            blast_speed: 100.0,
            max_health: HEALTH,
            health: HEALTH,
            knockback_max_speed: MAX_KNOCKBACK_SPEED,
            steel_barrel: false,
            collision_damage: COLLISION_DAMAGE,
        })
        .with(local_transform)
        .with(Transparent)
        .build();
}