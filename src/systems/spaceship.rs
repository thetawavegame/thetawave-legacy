use amethyst::{
    core::{
        Transform,
        timing::Time,
        nalgebra::Vector3,
    },
    ecs::{Join, Read, System, WriteStorage, Entities, LazyUpdate, ReadExpect},
    input::InputHandler,
};

use crate::{
    entities::{fire_blast, spawn_enemy},
    components::{Spaceship, Enemy},
    resources::{BlastResource, EnemyResource},
    space_shooter::GAME_HEIGHT,
};


pub struct SpaceshipSystem;
impl<'s> System<'s> for SpaceshipSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Spaceship>,
        WriteStorage<'s, Enemy>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
        ReadExpect<'s, BlastResource>,
        ReadExpect<'s, EnemyResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut transforms, mut spaceships, mut enemies, input, time, sprite_resource, enemy_resource, lazy_update): Self::SystemData) {

        //check if keys for different actions are down
        let mut x_move = input.axis_value("player_x").unwrap() as f32;
        let mut y_move = input.axis_value("player_y").unwrap() as f32;
        let mut shoot = input.action_is_down("shoot").unwrap();
        let spawn = input.action_is_down("spawn_enemy").unwrap();
        let mut barrel_left = input.action_is_down("barrel_left").unwrap();
        let mut barrel_right= input.action_is_down("barrel_right").unwrap();


        for (spaceship, transform) in (&mut spaceships, &mut transforms).join() {

            //firing cooldown
            if spaceship.fire_reset_timer > 0.0 {
                spaceship.fire_reset_timer -= time.delta_seconds();
                shoot = false;
            }

            //barrel roll input cooldown
            if spaceship.barrel_reset_timer > 0.0 && !spaceship.barrel_action_left && !spaceship.barrel_action_right {
                spaceship.barrel_reset_timer -= time.delta_seconds();
                barrel_left = false;
                barrel_right = false;
            }

            //barrel roll action cooldown
            if spaceship.barrel_action_left || spaceship.barrel_action_right {

                //if currently barrel rolling can't initiate another barrel roll
                barrel_left = false;
                barrel_right = false;

                //countdown to end of barrel roll if time left else set velocity to the appropriate max speed, stop the action, and reset cooldown
                if spaceship.barrel_action_timer > 0.0 {
                    spaceship.barrel_action_timer -= time.delta_seconds();
                }else {


                    if spaceship.barrel_action_left {
                        spaceship.current_velocity_x = -1.0 * spaceship.max_speed;
                    }

                    if spaceship.barrel_action_right {
                        spaceship.current_velocity_x = spaceship.max_speed;
                    }

                    spaceship.barrel_action_left = false;
                    spaceship.barrel_action_right = false;
                    spaceship.barrel_reset_timer = spaceship.barrel_cooldown;
                }

                //damage enemies in barrel roll
                /*
                for (enemy, enemy_transform) in (&mut enemies, &transforms).join() {

                    let spaceship_x = transform.translation().x;
                    let spaceship_y = transform.translation().y;
                    let enemy_x = enemy_transform.translation().x;
                    let enemy_y = enemy_transform.translation().y;

                    if hitbox_collide(spaceship_x, spaceship_y, enemy_x, enemy_y, spaceship.hitbox_width, spaceship.hitbox_height, enemy.hitbox_width, enemy.hitbox_height) {
                        enemy.health -= spaceship.barrel_damage;
                    }
                }
                */
            }

            let spaceship_x = transform.translation().x;
            let spaceship_y = transform.translation().y;

            //if barrel rolling a direction use the barrel roll x velocity, otherwise accelerate normally
            if spaceship.barrel_action_left {

                transform.set_x(spaceship_x - (spaceship.barrel_speed) * time.delta_seconds());
                transform.set_y(spaceship_y + (spaceship.current_velocity_y) * time.delta_seconds());

            } else if spaceship.barrel_action_right {

                transform.set_x(spaceship_x + (spaceship.barrel_speed) * time.delta_seconds());
                transform.set_y(spaceship_y + (spaceship.current_velocity_y) * time.delta_seconds());

            } else {
                //conditions for acceleration and deceleration in x
                if x_move > 0.0 && spaceship.current_velocity_x < spaceship.max_speed {
                    spaceship.current_velocity_x += spaceship.acceleration_x;
                } else if x_move < 0.0 && spaceship.current_velocity_x > (-1.0 * spaceship.max_speed) {
                    spaceship.current_velocity_x -= spaceship.acceleration_x;
                } else if x_move == 0.0 && spaceship.current_velocity_x > 0.0 {
                    spaceship.current_velocity_x -= spaceship.deceleration_x;
                } else if x_move == 0.0 && spaceship.current_velocity_x < 0.0 {
                    spaceship.current_velocity_x += spaceship.deceleration_x;
                }

                //conditions for acceleration and deceleration in y
                if y_move > 0.0 && spaceship.current_velocity_y < spaceship.max_speed {
                    spaceship.current_velocity_y += spaceship.acceleration_y;
                } else if y_move < 0.0 && spaceship.current_velocity_y > (-1.0 * spaceship.max_speed) {
                    spaceship.current_velocity_y -= spaceship.acceleration_y;
                } else if y_move == 0.0 && spaceship.current_velocity_y > 0.0 {
                    spaceship.current_velocity_y -= spaceship.deceleration_y;
                } else if y_move == 0.0 && spaceship.current_velocity_y < 0.0 {
                    spaceship.current_velocity_y += spaceship.deceleration_y;
                }

                //transform the spaceship in x and y by the currrent velocity in x and y
                transform.set_x(spaceship_x + (spaceship.current_velocity_x) * time.delta_seconds());
                transform.set_y(spaceship_y + (spaceship.current_velocity_y) * time.delta_seconds());


            }

            if shoot && !spaceship.barrel_action_left && !spaceship.barrel_action_right {
                let fire_position = Vector3::new(
                    transform.translation()[0], transform.translation()[1] + spaceship.height / 2.0, 0.1,
                );

                fire_blast(&entities, &sprite_resource, fire_position, spaceship.damage, spaceship.current_velocity_x, spaceship.current_velocity_y, &lazy_update);
                spaceship.fire_reset_timer = spaceship.fire_speed;
            }

            if barrel_left {
                spaceship.barrel_action_left = true;
                spaceship.barrel_action_timer = spaceship.barrel_duration;
            }

            if barrel_right {
                spaceship.barrel_action_right = true;
                spaceship.barrel_action_timer = spaceship.barrel_duration;
            }
        }
    }
}

fn hitbox_collide(mut x1: f32, mut y1: f32, mut x2: f32, mut y2: f32, hitbox_width_1: f32, hitbox_height_1: f32, hitbox_width_2: f32, hitbox_height_2: f32) -> bool {

    x1 -= hitbox_width_1 / 2.0;
    y1 -= hitbox_height_1 / 2.0;

    x2 -= hitbox_width_2 / 2.0;
    y2 -= hitbox_height_2 / 2.0 ;


    x1 < (x2 + hitbox_width_2) && (x1 + hitbox_width_1) > x2 && y1 < (y2 + hitbox_height_2) && (y1 + hitbox_height_1) > y2
    //(x1 + hitbox_width_1 / 2.0) > (x2 - hitbox_width_2 / 2.0) && (x1 - hitbox_width_1 / 2.0) < (x2 + hitbox_width_2 / 2.0) && (y1 - hitbox_height_1 / 2.0) > (y2 + hitbox_height_2 / 2.0) && (y1 + hitbox_height_1 / 2.0) < (y2 - hitbox_height_2)
}

