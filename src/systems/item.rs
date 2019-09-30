use amethyst::{
    core::{
        transform::Transform,
        timing::Time,
    },
    ecs::prelude::{Entities, Join, System, WriteStorage, Read, ReadExpect},
    audio::{output::Output, Source},
    assets::AssetStorage,
};
use std::ops::Deref;
use crate::{
    components::{Item, Spaceship, Defense},
    systems::hitbox_collide,
    audio::{play_sfx, Sounds},
};

use crate::space_shooter::ARENA_MIN_Y;
use crate::components::Living;

pub struct ItemSystem;

impl<'s> System<'s> for ItemSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Item>,
        WriteStorage<'s, Spaceship>,
        WriteStorage<'s, Defense>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>
    );

    fn run(&mut self, (entities, mut items, mut spaceships, mut defenses, mut transforms, time, storage, sounds, audio_output): Self::SystemData) {
        for (item_entity, item, item_transform) in (&*entities, &mut items, &mut transforms).join() {
            let item_x = item_transform.translation().x;
            let item_y = item_transform.translation().y;

            for spaceship in (&mut spaceships).join() {
                if hitbox_collide(item_x, item_y, spaceship.pos_x, spaceship.pos_y, item.hitbox_width, item.hitbox_height, spaceship.hitbox_width, spaceship.hitbox_height) {

                    if item.stat_effects.contains_key("max_defense") {
                        for defense in (&mut defenses).join() {
                            defense.set_max_health(defense.max_health() + item.stat_effects["max_defense"]);
                            defense.set_health(defense.health() + item.stat_effects["max_defense"]);
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

                    play_sfx(&sounds.item_sfx, &storage, audio_output.as_ref().map(|o| o.deref()));

                    let _result = entities.delete(item_entity);
                }else {
                    item_transform.prepend_translation_y(-1.0 * item.speed * time.delta_seconds());

                    if item_transform.translation()[1] + item.height/2.0 < ARENA_MIN_Y {
                        let _result = entities.delete(item_entity);
                    }
                }
            }
        }
    }
}

