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
    components::{Spaceship, Enemy, Fires, Living},
    resources::{SpriteResource},
};

const PLAYER_BLAST_Z: f32 = 0.8;
const PLAYER_BLAST_SPRITE_INDEX: usize = 3;

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

        let shoot_action = input.action_is_down("shoot").unwrap();
        let mut barrel_left = input.action_is_down("barrel_left").unwrap();
        let mut barrel_right= input.action_is_down("barrel_right").unwrap();


        for (spaceship, transform) in (&mut spaceships, &mut transforms).join() {

            spaceship.pos_x = transform.translation().x;
            spaceship.pos_y= transform.translation().y;

            //firing cooldown
            let fire_cooldown = spaceship.fire_cooldown(time.delta_seconds());

            //barrel roll input cooldown
            if spaceship.barrel_input_cooldown(time.delta_seconds()) {
                barrel_left = false;
                barrel_right = false;
            }

            //barrel roll action cooldown
            if spaceship.barrel_action_cooldown(time.delta_seconds()) {
                barrel_left = false;
                barrel_right = false;
            }
            
            //conditions for firing a blast
            if !fire_cooldown && shoot_action && !spaceship.barrel_action_left && !spaceship.barrel_action_right {
                let fire_position = Vector3::new(
                    transform.translation()[0], transform.translation()[1] + spaceship.height / 2.0, PLAYER_BLAST_Z,
                );

                fire_blast(&entities, &sprite_resource, PLAYER_BLAST_SPRITE_INDEX, fire_position, spaceship.damage, spaceship.current_velocity_x, spaceship.current_velocity_y, spaceship.blast_speed, true, &lazy_update);
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

            spaceship.constrain_health(); 
        }
    }
}
