use crate::{
    audio::Sounds,
    components::{
        BlasterComponent, HealthComponent, ManualFireComponent, Motion2DComponent, Spaceship,
    },
    events::PlayAudioEvent,
    resources::SpriteResource,
};
use amethyst::{
    core::{timing::Time, Transform},
    ecs::{Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    shrev::EventChannel,
};

pub struct SpaceshipSystem;

impl<'s> System<'s> for SpaceshipSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Spaceship>,
        WriteStorage<'s, HealthComponent>,
        WriteStorage<'s, Motion2DComponent>,
        ReadStorage<'s, BlasterComponent>,
        WriteStorage<'s, ManualFireComponent>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
        Write<'s, EventChannel<PlayAudioEvent>>,
        ReadExpect<'s, Sounds>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut transforms,
            mut spaceships,
            mut healths,
            mut motion2d_components,
            blasters,
            mut manualfire,
            input,
            time,
            sprite_resource,
            lazy_update,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        // collect input bools
        let shoot_action = input.action_is_down("shoot").unwrap();
        let mut barrel_left = input.action_is_down("barrel_left").unwrap();
        let mut barrel_right = input.action_is_down("barrel_right").unwrap();

        for (spaceship, health, transform, motion2d, blaster, manualfire) in (
            &mut spaceships,
            &mut healths,
            &mut transforms,
            &mut motion2d_components,
            &blasters,
            &mut manualfire,
        )
            .join()
        {
            // update pos_x and pos_y variables of spaceship
            spaceship.update_location(transform.translation().x, transform.translation().y);

            // barrel roll input cooldown
            // amount of time until new barrel roll can be initiated
            if spaceship.barrel_input_cooldown(time.delta_seconds()) {
                barrel_left = false;
                barrel_right = false;
            }

            //barrel roll action cooldown
            //amount of time until barrel roll is complete
            if spaceship.barrel_action_cooldown(time.delta_seconds(), motion2d) {
                barrel_left = false;
                barrel_right = false;
            }

            if !spaceship.barrel_action_left
                && !spaceship.barrel_action_right
                && shoot_action
                && manualfire.ready
            {
                blaster.fire(
                    motion2d,
                    transform,
                    &entities,
                    &sprite_resource,
                    &lazy_update,
                );
                manualfire.ready = false;
                play_audio_channel.single_write(PlayAudioEvent {
                    source: sounds.spaceship_laser_sfx.clone(),
                });
            }

            spaceship.initiate_barrel_roll(barrel_left, barrel_right);
            health.constrain();
        }
    }
}
