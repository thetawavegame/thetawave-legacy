use amethyst::{
    prelude::Builder,
    ecs::prelude::World,
    core::transform::Transform,
    renderer::{SpriteRender, SpriteSheetHandle, Transparent},
};

use crate::{
    components::Spaceship,
    space_shooter::{GAME_WIDTH, GAME_HEIGHT},
};


const SPACESHIP_HEIGHT: f32 = 18.0;
const SPACESHIP_WIDTH: f32 = 18.0;
const SPACESHIP_HITBOX_HEIGHT: f32 = 14.0;
const SPACESHIP_HITBOX_WIDTH: f32 = 14.0;
const SPACESHIP_ACCELERATION_X: f32 = 2.0;
const SPACESHIP_DECELERATION_X: f32 = 1.0;
const SPACESHIP_ACCELERATION_Y: f32 = 4.0;
const SPACESHIP_DECELERATION_Y: f32 = 1.0;
const SPACESHIP_MAX_SPEED: f32 = 70.0;
const SPACESHIP_STARTING_FIRE_SPEED: f32 = 0.3;
const SPACESHIP_STARTING_DAMAGE: f32 = 40.0;
const SPACESHIP_BARREL_COOLDOWN: f32 = 0.5;
const SPACESHIP_BARREL_SPEED: f32 = 180.0;
const SPACESHIP_BARREL_DURATION: f32 = 0.3;


pub fn initialise_spaceship(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {

    let mut local_transform = Transform::default();
    local_transform.set_xyz(GAME_WIDTH / 2.0, GAME_HEIGHT / 6.0, 0.9);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Spaceship {
            width: SPACESHIP_WIDTH,
            height: SPACESHIP_HEIGHT,
            hitbox_width: SPACESHIP_HITBOX_WIDTH,
            hitbox_height: SPACESHIP_HITBOX_HEIGHT,
            max_speed: SPACESHIP_MAX_SPEED,
            current_velocity_x: 0.0,
            current_velocity_y: 0.0,
            acceleration_x: SPACESHIP_ACCELERATION_X,
            deceleration_x: SPACESHIP_DECELERATION_X,
            acceleration_y: SPACESHIP_ACCELERATION_Y,
            deceleration_y: SPACESHIP_DECELERATION_Y,
            fire_speed: SPACESHIP_STARTING_FIRE_SPEED,
            fire_reset_timer: 0.0,
            damage: SPACESHIP_STARTING_DAMAGE,
            barrel_cooldown: SPACESHIP_BARREL_COOLDOWN,
            barrel_reset_timer: 0.0,
            barrel_speed: SPACESHIP_BARREL_SPEED,
            barrel_action_right: false,
            barrel_action_left: false,
            barrel_duration: SPACESHIP_BARREL_DURATION,
            barrel_action_timer: SPACESHIP_BARREL_DURATION,
            barrel_damage: 0.0,
            pos_x: local_transform.translation().x,
            pos_y: local_transform.translation().y,
        })
        .with(local_transform)
        .with(Transparent)
        .build();
}