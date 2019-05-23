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
    components::Spaceship,
    resources::{BlastResource, EnemyResource},
    space_shooter::GAME_HEIGHT,
};


pub struct SpaceshipSystem;
impl<'s> System<'s> for SpaceshipSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Spaceship>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
        ReadExpect<'s, BlastResource>,
        ReadExpect<'s, EnemyResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut transforms, mut spaceships, input, time, sprite_resource, enemy_resource, lazy_update): Self::SystemData) {


        let mut x_move = input.axis_value("player_x").unwrap() as f32;
        let mut y_move = input.axis_value("player_y").unwrap() as f32;
        let mut shoot = input.action_is_down("shoot").unwrap();
        let spawn = input.action_is_down("spawn_enemy").unwrap();
        let mut barrel_left = input.action_is_down("barrel_left").unwrap();
        let mut barrel_right= input.action_is_down("barrel_right").unwrap();

        let mut diag: f32 = 1.0;
        //if moving diaganal multiply by move values by sqrt2/2
        if x_move.abs() > 0.0 && y_move.abs() > 0.0 {
            diag = ((2.0 as f64).sqrt() / 2.0) as f32;
            //y_move = y_move * ((2.0 as f64).sqrt() / 2.0) as  f32;
        }




        for (spaceship, transform) in (&mut spaceships, &mut transforms).join() {

            if spaceship.fire_reset_timer > 0.0 {
                spaceship.fire_reset_timer -= time.delta_seconds();
                shoot = false;
            }

            if spaceship.barrel_reset_timer > 0.0 {
                spaceship.barrel_reset_timer -= time.delta_seconds();
                barrel_left = false;
                barrel_right = false;
            }

            if spaceship.barrel_action_left || spaceship.barrel_action_right {
                println!("in action");
                barrel_left = false;
                barrel_right = false;

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

            }


            let spaceship_x = transform.translation().x;
            let spaceship_y = transform.translation().y;

            //accelerate
            if spaceship.barrel_action_left {

                transform.set_x(spaceship_x - (spaceship.barrel_speed) * time.delta_seconds());

            } else if spaceship.barrel_action_right {

                transform.set_x(spaceship_x + (spaceship.barrel_speed) * time.delta_seconds());

            } else {
                if x_move > 0.0 && spaceship.current_velocity_x < spaceship.max_speed {
                    spaceship.current_velocity_x += spaceship.acceleration_x;
                } else if x_move < 0.0 && spaceship.current_velocity_x > (-1.0 * spaceship.max_speed) {
                    spaceship.current_velocity_x -= spaceship.acceleration_x;
                } else if x_move == 0.0 && spaceship.current_velocity_x > 0.0 {
                    spaceship.current_velocity_x -= spaceship.deceleration_x;
                } else if x_move == 0.0 && spaceship.current_velocity_x < 0.0 {
                    spaceship.current_velocity_x += spaceship.deceleration_x;
                }

                if y_move > 0.0 && spaceship.current_velocity_y < spaceship.max_speed {
                    spaceship.current_velocity_y += spaceship.acceleration_y;
                } else if y_move < 0.0 && spaceship.current_velocity_y > (-1.0 * spaceship.max_speed) {
                    spaceship.current_velocity_y -= spaceship.acceleration_y;
                } else if y_move == 0.0 && spaceship.current_velocity_y > 0.0 {
                    spaceship.current_velocity_y -= spaceship.deceleration_y;
                } else if y_move == 0.0 && spaceship.current_velocity_y < 0.0 {
                    spaceship.current_velocity_y += spaceship.deceleration_y;
                }

                transform.set_x(spaceship_x + (spaceship.current_velocity_x) * time.delta_seconds());
                transform.set_y(spaceship_y + (spaceship.current_velocity_y) * time.delta_seconds());

                spaceship.x_velocity = spaceship.current_velocity_x;
                spaceship.y_velocity = spaceship.current_velocity_y;


                if shoot {
                    println!("Shoot!");

                    let fire_position = Vector3::new(
                        transform.translation()[0], transform.translation()[1] + spaceship.height / 2.0, 0.1,
                    );

                    fire_blast(&entities, &sprite_resource, fire_position, spaceship.damage, spaceship.x_velocity, spaceship.y_velocity, &lazy_update);
                    spaceship.fire_reset_timer = spaceship.fire_speed;
                }
            }

            if barrel_left {
                println!("barrel_left");
                //spaceship.barrel_reset_timer = spaceship.barrel_cooldown;
                spaceship.barrel_action_left = true;
                spaceship.barrel_action_timer = spaceship.barrel_duration;
            }

            if barrel_right {
                println!("barrel_right");
                //spaceship.barrel_reset_timer = spaceship.barrel_cooldown;
                spaceship.barrel_action_right = true;
                spaceship.barrel_action_timer = spaceship.barrel_duration;
            }



        }
    }
}
