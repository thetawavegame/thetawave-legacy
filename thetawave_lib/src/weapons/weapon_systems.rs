use crate::{
    audio::Sounds,
    events::PlayAudioEvent,
    motion::components::Motion2DComponent,
    visual::resources::SpriteSheetsResource,
    weapons::{AutoFireComponent, BlasterComponent, ManualFireComponent},
};
use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{
        Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, Write, WriteStorage,
    },
    input::{InputHandler, StringBindings},
    shrev::EventChannel,
};

/// Handles periodic firing of weapons
pub struct AutoFireSystem;

impl<'s> System<'s> for AutoFireSystem {
    /// Data used by the system
    type SystemData = (
        Entities<'s>,
        Read<'s, Time>,
        ReadExpect<'s, LazyUpdate>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, BlasterComponent>,
        WriteStorage<'s, AutoFireComponent>,
        ReadStorage<'s, Motion2DComponent>,
        ReadExpect<'s, SpriteSheetsResource>,
    );

    /// System game logic
    fn run(
        &mut self,
        (
            entities,
            time,
            lazy_update,
            transforms,
            blasters,
            mut auto_fires,
            motion2ds,
            sprite_resource,
        ): Self::SystemData,
    ) {
        for (transform, auto_fire, blaster, motion2d) in
            (&transforms, &mut auto_fires, &blasters, &motion2ds).join()
        {
            if auto_fire.update(time.delta_seconds()) {
                blaster.fire(
                    motion2d,
                    transform,
                    &entities,
                    &sprite_resource,
                    &lazy_update,
                );
            }
        }
    }
}

/// Handles firing of weapons using input
pub struct ManualBlasterSystem;

impl<'s> System<'s> for ManualBlasterSystem {
    /// Data used by the system
    type SystemData = (
        Read<'s, Time>,
        WriteStorage<'s, ManualFireComponent>,
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Motion2DComponent>,
        ReadStorage<'s, BlasterComponent>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, SpriteSheetsResource>,
        ReadExpect<'s, LazyUpdate>,
        Write<'s, EventChannel<PlayAudioEvent>>,
        ReadExpect<'s, Sounds>,
    );

    /// System game logic
    fn run(
        &mut self,
        (
            time,
            mut manual_fires,
            entities,
            mut transforms,
            mut motion2ds,
            blasters,
            input,
            sprite_resource,
            lazy_update,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        let player_fire_action = input.action_is_down("shoot").unwrap();

        for (transform, motion2d, blaster, manual_fire) in (
            &mut transforms,
            &mut motion2ds,
            &blasters,
            &mut manual_fires,
        )
            .join()
        {
            manual_fire.update(time.delta_seconds());

            if player_fire_action && manual_fire.fire() {
                blaster.fire(
                    motion2d,
                    transform,
                    &entities,
                    &sprite_resource,
                    &lazy_update,
                );
                play_audio_channel.single_write(PlayAudioEvent {
                    source: sounds.sound_effects["laser_blast"].clone(),
                })
            }
        }
    }
}
