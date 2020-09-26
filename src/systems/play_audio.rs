use crate::{
    audio::{play_sfx, Sounds},
    constants::{
        CASH_REGISTER_BELL, CRASH_SFX, EXPLOSION_SFX, ITEM_SFX, LARGE_ROCK_SFX, SMALL_ROCK_SFX,
        SPACESHIP_HIT_SFX, SPACESHIP_LASER_SFX, WRENCH_SFX,
    },
    events::PlayAudioEvent,
};
use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    ecs::*,
    ecs::{Read, ReadExpect, System, World},
    shrev::{EventChannel, ReaderId},
};

#[derive(Default)]
pub struct PlayAudioSystem {
    event_reader: Option<ReaderId<PlayAudioEvent>>,
}

impl<'s> System<'s> for PlayAudioSystem {
    type SystemData = (
        Read<'s, EventChannel<PlayAudioEvent>>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<PlayAudioEvent>>()
                .register_reader(),
        );
    }

    fn run(&mut self, (play_audio_event_channel, storage, sounds, audio_output): Self::SystemData) {
        for event in play_audio_event_channel.read(self.event_reader.as_mut().unwrap()) {
            match event.value {
                SMALL_ROCK_SFX => {
                    play_sfx(&sounds.small_rock_sfx, &storage, audio_output.as_deref());
                }
                LARGE_ROCK_SFX => {
                    play_sfx(&sounds.large_rock_sfx, &storage, audio_output.as_deref());
                }
                WRENCH_SFX => {
                    play_sfx(&sounds.wrench_sfx, &storage, audio_output.as_deref());
                }
                ITEM_SFX => {
                    play_sfx(&sounds.item_sfx, &storage, audio_output.as_deref());
                }
                SPACESHIP_LASER_SFX => {
                    play_sfx(
                        &sounds.spaceship_laser_sfx,
                        &storage,
                        audio_output.as_deref(),
                    );
                }
                CRASH_SFX => {
                    play_sfx(&sounds.crash_sfx, &storage, audio_output.as_deref());
                }
                EXPLOSION_SFX => {
                    play_sfx(&sounds.explosion_sfx, &storage, audio_output.as_deref());
                }
                SPACESHIP_HIT_SFX => {
                    play_sfx(&sounds.spaceship_hit_sfx, &storage, audio_output.as_deref());
                }
                CASH_REGISTER_BELL => {
                    play_sfx(
                        &sounds.cash_register_bell,
                        &storage,
                        audio_output.as_deref(),
                    );
                }
                _ => {}
            }
        }
    }
}
