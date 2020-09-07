use crate::{
    audio::{play_sfx, Sounds},
    components::{Defense, Item, Living, Spaceship},
    constants::ARENA_MIN_Y,
    space_shooter::HitboxCollisionEvent,
};
use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Entities, Join, ReadExpect, WriteStorage},
    ecs::*,
    ecs::{Read, System, World},
    shrev::{EventChannel, ReaderId},
};

#[derive(Default)]
pub struct ItemSystem {
    event_reader: Option<ReaderId<HitboxCollisionEvent>>,
}

impl<'s> System<'s> for ItemSystem {
    type SystemData = (
        Read<'s, EventChannel<HitboxCollisionEvent>>,
        Entities<'s>,
        WriteStorage<'s, Item>,
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
            mut items,
            mut spaceships,
            mut defenses,
            mut transforms,
            time,
            storage,
            sounds,
            audio_output,
        ): Self::SystemData,
    ) {
        // item movement
        for (item, item_entity, item_transform) in (&mut items, &entities, &mut transforms).join() {
            item_transform.prepend_translation_y(-1.0 * item.speed * time.delta_seconds());

            if item_transform.translation().y + item.height / 2.0 < ARENA_MIN_Y {
                entities.delete(item_entity);
            }
        }

        // event loop needs to be outer so that all events are always read
        for event in collision_channel.read(self.event_reader.as_mut().unwrap()) {
            for (item, item_entity) in (&mut items, &entities).join() {
                for (spaceship, spaceship_entity) in (&mut spaceships, &entities).join() {
                    //println!("{:?}", event);
                    if (event.entity_a == item_entity && event.entity_b == spaceship_entity)
                        || (event.entity_a == spaceship_entity && event.entity_b == item_entity)
                    {
                        if item.stat_effects.contains_key("max_defense") {
                            for defense in (&mut defenses).join() {
                                // increases maximum capacity for defense
                                defense.set_max_health(
                                    defense.max_health() + item.stat_effects["max_defense"],
                                );
                                // sets current defense to new maximum value
                                defense.set_health(
                                    defense.health() + item.stat_effects["max_defense"],
                                );
                            }
                        }

                        //add stats to spaceship
                        if item.bool_effects.contains_key("barrel_immunity") {
                            spaceship.steel_barrel = item.bool_effects["barrel_immunity"];
                        }

                        if item.stat_effects.contains_key("blast_count") {
                            spaceship.blast_count += item.stat_effects["blast_count"] as usize;
                        }

                        if item.stat_effects.contains_key("fire_speed") {
                            spaceship.fire_speed += item.stat_effects["fire_speed"];
                        }

                        if item.stat_effects.contains_key("damage") {
                            spaceship.damage += item.stat_effects["damage"];
                        }

                        if item.stat_effects.contains_key("max_speed") {
                            spaceship.max_speed += item.stat_effects["max_speed"];
                        }
                        if item.stat_effects.contains_key("crit_chance") {
                            spaceship.crit_chance += item.stat_effects["crit_chance"];
                        }

                        if item.stat_effects.contains_key("poison_chance") {
                            spaceship.poison_chance += item.stat_effects["poison_chance"];
                        }

                        if item.stat_effects.contains_key("barrel_cooldown") {
                            spaceship.barrel_cooldown += item.stat_effects["barrel_cooldown"];
                        }

                        if item.stat_effects.contains_key("acceleration") {
                            spaceship.acceleration_x += item.stat_effects["acceleration"];
                            spaceship.acceleration_y += item.stat_effects["acceleration"];
                        }

                        if item.stat_effects.contains_key("deceleration") {
                            spaceship.deceleration_x += item.stat_effects["deceleration"];
                            spaceship.deceleration_y += item.stat_effects["deceleration"];
                        }

                        if item.stat_effects.contains_key("health_multiply") {
                            spaceship.set_max_health(
                                spaceship.max_health()
                                    + (item.stat_effects["health_multiply"] * spaceship.max_health),
                            );
                            spaceship.set_health(spaceship.max_health)
                        }

                        if item.stat_effects.contains_key("health_add") {
                            spaceship.set_max_health(
                                spaceship.max_health() + item.stat_effects["health_add"],
                            );
                            spaceship.set_health(spaceship.max_health);
                        }

                        play_sfx(&sounds.item_sfx, &storage, audio_output.as_deref());

                        entities.delete(item_entity);
                    }
                }
            }
        }
    }
}
