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
    entities::{fire_blast},
    components::{Spaceship, Enemy},
    resources::{SpriteResource},
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
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut transforms, mut spaceships, mut enemies, input, time, sprite_resource, lazy_update): Self::SystemData) {

        let mut shoot = input.action_is_down("shoot").unwrap();
        let mut barrel_left = input.action_is_down("barrel_left").unwrap();
        let mut barrel_right= input.action_is_down("barrel_right").unwrap();


        for (spaceship, transform) in (&mut spaceships, &mut transforms).join() {

            spaceship.pos_x = transform.translation().x;
            spaceship.pos_y= transform.translation().y;

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
                    for enemy in (&mut enemies).join() {
                        enemy.barrel_damaged = false;
                    }

                }

            }

            if shoot && !spaceship.barrel_action_left && !spaceship.barrel_action_right {
                let fire_position = Vector3::new(
                    transform.translation()[0], transform.translation()[1] + spaceship.height / 2.0, 0.1,
                );

                fire_blast(&entities, &sprite_resource, 3, fire_position, spaceship.damage, spaceship.current_velocity_x, spaceship.current_velocity_y, &lazy_update);
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
