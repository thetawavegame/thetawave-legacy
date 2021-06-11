use crate::{
    audio::Sounds,
    components::HealthComponent,
    entities::{EffectType, SpawnableType},
    events::{MobDestroyedEvent, PlayAudioEvent},
    resources::{DropTablesResource, SpriteSheetsResource},
    spawnable::{
        components::MobComponent,
        entities::spawn_effect,
        resources::{ConsumablesResource, EffectsResource, ItemsResource, MobsResource},
    },
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
            mob_health.constrain();

            // conditions for despawning
            if mob_health.value <= 0.0 {
                mob_destroyed_event_channel.single_write(MobDestroyedEvent::new(mob_entity));
            }
        }
    }
}

#[derive(Default)]
pub struct MobDestroyedSystem {
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

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<MobDestroyedEvent>>()
                .register_reader(),
        );
    }

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
        for event in mob_destroyed_event_channel.read(self.event_reader.as_mut().unwrap()) {
            let mob_transform = transforms.get(event.mob).unwrap();
            let mob_component = mobs.get(event.mob).unwrap();

            play_audio_channel.single_write(PlayAudioEvent {
                source: sounds.sound_effects["explosion"].clone(),
            });

            spawn_effect(
                &EffectType::MobExplosion,
                mob_transform.clone(),
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );

            if let SpawnableType::Mob(mob_type) = mob_component.spawnable_type.clone() {
                if effects_resource
                    .get(&EffectType::Giblets(mob_type.clone()))
                    .is_some()
                {
                    spawn_effect(
                        &EffectType::Giblets(mob_type),
                        mob_transform.clone(),
                        &effects_resource,
                        &spritesheets_resource,
                        &entities,
                        &lazy_update,
                    );
                }
            }
            mob_component.drop_rolls.spawn(
                mob_transform.clone(),
                &drop_tables_resource,
                &consumables_resource,
                &mobs_resource,
                &items_resource,
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );

            entities.delete(event.mob).expect("unable to delete entity");
        }
    }
}
