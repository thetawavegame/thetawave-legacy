use crate::{
    audio::{play_sfx, Sounds},
    components::{choose_random_name, Enemy},
    entities::{spawn_consumable, spawn_explosion},
    events::EnemyDestroyedEvent,
    resources::{ConsumableEntityData, SpriteResource},
};
use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::transform::Transform,
    ecs::prelude::{Entities, Join, LazyUpdate, ReadExpect, ReadStorage, System},
    ecs::*,
    ecs::{Read, World},
    shrev::{EventChannel, ReaderId},
};
use std::collections::HashMap;

#[derive(Default)]
pub struct EnemyDestroyedSystem {
    event_reader: Option<ReaderId<EnemyDestroyedEvent>>,
}

impl<'s> System<'s> for EnemyDestroyedSystem {
    type SystemData = (
        Read<'s, EventChannel<EnemyDestroyedEvent>>,
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Enemy>,
        ReadExpect<'s, HashMap<String, ConsumableEntityData>>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
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
            consumable_pool,
            sprite_resource,
            lazy_update,
            storage,
            sounds,
            audio_output,
        ): Self::SystemData,
    ) {
        for event in enemy_destroyed_event_channel.read(self.event_reader.as_mut().unwrap()) {
            for (enemy_entity, enemy_component, enemy_transform) in
                (&entities, &enemies, &transforms).join()
            {
                if event.enemy == enemy_entity {
                    play_sfx(&sounds.explosion_sfx, &storage, audio_output.as_deref());

                    spawn_explosion(
                        &entities,
                        &sprite_resource,
                        enemy_component.explosion_sprite_idx,
                        enemy_transform.translation(),
                        &lazy_update,
                    );

                    let name = choose_random_name(&enemy_component.collectables_probs);
                    if !name.is_empty() {
                        spawn_consumable(
                            &entities,
                            &sprite_resource,
                            consumable_pool[name].clone(),
                            enemy_transform.translation(),
                            &lazy_update,
                        );
                    }

                    entities
                        .delete(enemy_entity)
                        .expect("unable to delete entity");
                }
            }
        }
    }
}
