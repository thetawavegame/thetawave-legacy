use crate::{
    audio::Sounds,
    components::EnemyComponent,
    entities::{spawn_effect, spawn_random_consumable, EffectType},
    events::{EnemyDestroyedEvent, PlayAudioEvent},
    resources::{ConsumablesResource, EffectsResource, SpriteSheetsResource},
};
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities, LazyUpdate, ReadExpect, ReadStorage, System},
    ecs::*,
    ecs::{Read, World},
    shrev::{EventChannel, ReaderId},
};

#[derive(Default)]
pub struct EnemyDestroyedSystem {
    event_reader: Option<ReaderId<EnemyDestroyedEvent>>,
}

impl<'s> System<'s> for EnemyDestroyedSystem {
    type SystemData = (
        Read<'s, EventChannel<EnemyDestroyedEvent>>,
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, EnemyComponent>,
        ReadExpect<'s, ConsumablesResource>,
        ReadExpect<'s, EffectsResource>,
        ReadExpect<'s, SpriteSheetsResource>,
        ReadExpect<'s, LazyUpdate>,
        Write<'s, EventChannel<PlayAudioEvent>>,
        ReadExpect<'s, Sounds>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<EnemyDestroyedEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (
            enemy_destroyed_event_channel,
            entities,
            transforms,
            enemies,
            consumables_resource,
            effects_resource,
            sprite_resource,
            lazy_update,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        for event in enemy_destroyed_event_channel.read(self.event_reader.as_mut().unwrap()) {
            let enemy_transform = transforms.get(event.enemy).unwrap();
            let enemy_component = enemies.get(event.enemy).unwrap();

            play_audio_channel.single_write(PlayAudioEvent {
                source: sounds.sound_effects["explosion"].clone(),
            });

            spawn_effect(
                &EffectType::EnemyExplosion,
                *enemy_transform.translation(),
                &effects_resource,
                &sprite_resource,
                &entities,
                &lazy_update,
            );

            spawn_random_consumable(
                &entities,
                &enemy_component,
                &sprite_resource,
                &consumables_resource,
                enemy_transform.translation(),
                &lazy_update,
            );

            entities
                .delete(event.enemy)
                .expect("unable to delete entity");
        }
    }
}
