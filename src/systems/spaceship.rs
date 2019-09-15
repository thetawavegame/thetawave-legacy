use amethyst::{
    core::{
        Transform,
        timing::Time,
    },
    ecs::{Join, Read, System, WriteStorage, Entities, LazyUpdate, ReadExpect},
    input::{InputHandler, StringBindings},
    audio::{output::Output, Source},
    assets::AssetStorage,
};
use std::ops::Deref;
use crate::{
    entities::{fire_blast, fire_double_blast},
    components::{Spaceship, Fires, Living},
    resources::{SpriteResource},
    audio::{play_sfx, Sounds},
};

const PLAYER_BLAST_SPRITE_INDEX: usize = 0;

pub struct SpaceshipSystem;

impl<'s> System<'s> for SpaceshipSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Spaceship>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>
    );

    fn run(&mut self, (entities, mut transforms, mut spaceships, input, time, sprite_resource, lazy_update, storage, sounds, audio_output): Self::SystemData) {

        let shoot_action = input.action_is_down("shoot").unwrap();
        let mut barrel_left = input.action_is_down("barrel_left").unwrap();
        let mut barrel_right= input.action_is_down("barrel_right").unwrap();


        for (spaceship, transform) in (&mut spaceships, &mut transforms).join() {

            spaceship.pos_x = transform.translation().x;
            spaceship.pos_y= transform.translation().y;

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
            
            if let Some(fire_position) = spaceship.fire_cooldown(transform, spaceship.height / 2.0, !spaceship.barrel_action_left && !spaceship.barrel_action_right && shoot_action, time.delta_seconds()) {
                if spaceship.double_blasts {
                    fire_double_blast(&entities, &sprite_resource, PLAYER_BLAST_SPRITE_INDEX, fire_position, spaceship.damage, spaceship.current_velocity_x, spaceship.current_velocity_y, spaceship.blast_speed, true, &lazy_update);
                }else {
                    fire_blast(&entities, &sprite_resource, PLAYER_BLAST_SPRITE_INDEX, fire_position, spaceship.damage, spaceship.current_velocity_x, spaceship.current_velocity_y, spaceship.blast_speed, true, &lazy_update);
                }
                play_sfx(&sounds.spaceship_laser_sfx, &storage, audio_output.as_ref().map(|o| o.deref()));
            }

            if barrel_left {
                spaceship.barrel_action_left = true;
                spaceship.barrel_action_timer = spaceship.barrel_duration;
                spaceship.barrel_reset_timer = spaceship.barrel_cooldown;
            }

            if barrel_right {
                spaceship.barrel_action_right = true;
                spaceship.barrel_action_timer = spaceship.barrel_duration;
                spaceship.barrel_reset_timer = spaceship.barrel_cooldown;
            }

            spaceship.constrain_health(); 
        }
    }
}
