use crate::{
    audio::{play_sfx, Sounds},
    components::{Consumable, Defense, Spaceship},
    constants::ARENA_MIN_Y,
    space_shooter::HitboxCollisionEvent,
};
use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Entities, Join, Read, ReadExpect, System, WriteStorage},
    ecs::*,
    shrev::{EventChannel, ReaderId},
};

#[derive(Default)]
pub struct ConsumableSystem {
    event_reader: Option<ReaderId<HitboxCollisionEvent>>,
}

impl<'s> System<'s> for ConsumableSystem {
    type SystemData = (
        Read<'s, EventChannel<HitboxCollisionEvent>>,
        Entities<'s>,
        WriteStorage<'s, Consumable>,
        WriteStorage<'s, Spaceship>,
        WriteStorage<'s, Defense>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<HitboxCollisionEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (
            collision_channel,
            entities,
            mut consumables,
            mut spaceships,
            mut defenses,
            mut transforms,
            time,
            storage,
            sounds,
            audio_output,
        ): Self::SystemData,
    ) {
        for (consumable, consumable_entity, consumable_transform) in
            (&mut consumables, &entities, &mut transforms).join()
        {
            consumable_transform
                .prepend_translation_y(-1.0 * consumable.speed * time.delta_seconds());

            if consumable_transform.translation()[1] < ARENA_MIN_Y {
                let _result = entities.delete(consumable_entity);
            }

            for (spaceship, spaceship_entity) in (&mut spaceships, &entities).join() {
                for event in collision_channel.read(self.event_reader.as_mut().unwrap()) {
                    if (event.entity_a == consumable_entity && event.entity_b == spaceship_entity)
                        || (event.entity_a == spaceship_entity
                            && event.entity_b == consumable_entity)
                    {
                        spaceship.health += consumable.health_value;
                        spaceship.money += consumable.money_value;

                        if consumable.money_value == 1 {
                            play_sfx(&sounds.small_rock_sfx, &storage, audio_output.as_deref());
                        } else if consumable.money_value == 5 {
                            play_sfx(&sounds.large_rock_sfx, &storage, audio_output.as_deref());
                        } else if consumable.health_value > 0.0 || consumable.defense_value > 0.0 {
                            play_sfx(&sounds.wrench_sfx, &storage, audio_output.as_deref());
                        }

                        for defense in (&mut defenses).join() {
                            defense.defense += consumable.defense_value;
                        }

                        let _result = entities.delete(consumable_entity);
                    }
                }
            }
        }
    }
}
