use crate::{
    audio::Sounds,
    components::HealthComponent,
    events::{ItemGetEvent, PlayAudioEvent},
    motion::components::Motion2DComponent,
    resources::SpriteSheetsResource,
    weapons::components::{BlasterComponent, ManualFireComponent},
};
use amethyst::{
    core::Transform,
    ecs::*,
    ecs::{Entities, Join, LazyUpdate, Read, ReadExpect, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    shrev::EventChannel,
};

#[derive(Default)]
pub struct SpaceshipSystem {
    item_get_event_reader: Option<ReaderId<ItemGetEvent>>,
}

impl<'s> System<'s> for SpaceshipSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, HealthComponent>,
        WriteStorage<'s, Motion2DComponent>,
        ReadStorage<'s, BlasterComponent>,
        WriteStorage<'s, ManualFireComponent>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, SpriteSheetsResource>,
        ReadExpect<'s, LazyUpdate>,
        Write<'s, EventChannel<PlayAudioEvent>>,
        ReadExpect<'s, Sounds>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.item_get_event_reader = Some(
            world
                .fetch_mut::<EventChannel<ItemGetEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (
            entities,
            mut transforms,
            mut healths,
            mut motion2ds,
            blasters,
            mut manual_fires,
            input,
            sprite_resource,
            lazy_update,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        // collect input bools
        let shoot_action = input.action_is_down("shoot").unwrap();

        for (health, transform, motion2d, blaster, manual_fire) in (
            &mut healths,
            &mut transforms,
            &mut motion2ds,
            &blasters,
            &mut manual_fires,
        )
            .join()
        {
            if shoot_action && manual_fire.ready {
                blaster.fire(
                    motion2d,
                    transform,
                    &entities,
                    &sprite_resource,
                    &lazy_update,
                );
                manual_fire.ready = false;
                play_audio_channel.single_write(PlayAudioEvent {
                    source: sounds.sound_effects["laser_blast"].clone(),
                });
            }

            health.constrain();
        }
    }
}
