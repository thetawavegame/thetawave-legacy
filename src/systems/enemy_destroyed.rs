use crate::{
    audio::Sounds,
    components::EnemyComponent,
    entities::{spawn_explosion, spawn_random_consumable},
    events::{EnemyDestroyedEvent, PlayAudioEvent},
    resources::{ConsumablesResource, SpriteSheetsResource},
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

            spawn_explosion(
                &entities,
                &sprite_resource,
                enemy_component.explosion_sprite_idx,
                enemy_transform.translation(),
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
