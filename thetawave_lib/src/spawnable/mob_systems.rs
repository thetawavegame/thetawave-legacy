use crate::{
    audio::Sounds,
    events::{MobDestroyedEvent, PlayAudioEvent},
    misc::HealthComponent,
    spawn::DropTablesResource,
    spawnable::{
        components::MobComponent, ConsumablesResource, EffectType, EffectsResource, ItemsResource,
        MobsResource, SpawnableResources,
    },
    visual::SpriteSheetsResource,
};
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{
        Entities, Join, LazyUpdate, ReadExpect, ReadStorage, System, Write, WriteStorage,
    },
    ecs::*,
    ecs::{Read, World},
    shrev::{EventChannel, ReaderId},
};

/// Handles health component of mobs
// TODO: remove and consolidate into health system
pub struct MobBehaviorSystem;

impl<'s> System<'s> for MobBehaviorSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, MobComponent>,
        WriteStorage<'s, HealthComponent>,
        Write<'s, EventChannel<MobDestroyedEvent>>,
    );

    fn run(
        &mut self,
        (entities, mut mobs, mut healths, mut mob_destroyed_event_channel): Self::SystemData,
    ) {
        for (mob_entity, _mob_component, mob_health) in (&*entities, &mut mobs, &mut healths).join()
        {
            mob_health.health.check(|| {
                mob_destroyed_event_channel.single_write(MobDestroyedEvent::new(mob_entity))
            });
        }
    }
}

/// Handles destruction of mob
#[derive(Default)]
pub struct MobDestroyedSystem {
    /// Reads from the mob destroyed event channel
    event_reader: Option<ReaderId<MobDestroyedEvent>>,
}

impl<'s> System<'s> for MobDestroyedSystem {
    type SystemData = (
        Read<'s, EventChannel<MobDestroyedEvent>>,
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, MobComponent>,
        ReadExpect<'s, DropTablesResource>,
        ReadExpect<'s, ConsumablesResource>,
        ReadExpect<'s, MobsResource>,
        ReadExpect<'s, EffectsResource>,
        ReadExpect<'s, ItemsResource>,
        ReadExpect<'s, SpriteSheetsResource>,
        ReadExpect<'s, LazyUpdate>,
        Write<'s, EventChannel<PlayAudioEvent>>,
        ReadExpect<'s, Sounds>,
    );

    fn run(
        &mut self,
        (
            mob_destroyed_event_channel,
            entities,
            transforms,
            mobs,
            drop_tables_resource,
            consumables_resource,
            mobs_resource,
            effects_resource,
            items_resource,
            spritesheets_resource,
            lazy_update,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        let spawnable_resources = &SpawnableResources {
            consumables_resource: &consumables_resource,
            mobs_resource: &mobs_resource,
            items_resource: &items_resource,
            effects_resource: &effects_resource,
        };

        for event in mob_destroyed_event_channel.read(self.event_reader.as_mut().unwrap()) {
            let mob_transform = transforms.get(event.mob).unwrap();
            let mob_component = mobs.get(event.mob).unwrap();

            play_audio_channel.single_write(PlayAudioEvent {
                source: sounds.sound_effects["explosion"].clone(),
            });

            effects_resource.spawn_effect(
                &EffectType::MobExplosion,
                mob_transform,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );

            let mob_type = &mob_component.mob_type;

            if effects_resource
                .effect_entities
                .get(&EffectType::Giblets(mob_type.clone()))
                .is_some()
            {
                effects_resource.spawn_effect(
                    &EffectType::Giblets(mob_type.clone()),
                    mob_transform,
                    &spritesheets_resource,
                    &entities,
                    &lazy_update,
                );
            }

            mob_component.drop_rolls.spawn(
                mob_transform,
                &drop_tables_resource,
                spawnable_resources,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );

            entities.delete(event.mob).expect("unable to delete entity");
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<MobDestroyedEvent>>()
                .register_reader(),
        );
    }
}
