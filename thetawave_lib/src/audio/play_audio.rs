use crate::{audio::play_sfx, events::PlayAudioEvent};
use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    ecs::*,
    ecs::{Read, System, World},
    shrev::{EventChannel, ReaderId},
};

/// Handles execution of sound effects
#[derive(Default)]
pub struct PlayAudioSystem {
    event_reader: Option<ReaderId<PlayAudioEvent>>,
}

impl<'s> System<'s> for PlayAudioSystem {
    type SystemData = (
        Read<'s, EventChannel<PlayAudioEvent>>,
        Read<'s, AssetStorage<Source>>,
        Option<Read<'s, Output>>,
    );

    fn run(&mut self, (play_audio_event_channel, storage, audio_output): Self::SystemData) {
        for event in play_audio_event_channel.read(self.event_reader.as_mut().unwrap()) {
            play_sfx(&event.source, &storage, audio_output.as_deref());
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<PlayAudioEvent>>()
                .register_reader(),
        );
    }
}
