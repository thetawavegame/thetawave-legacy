use crate::{
    audio::{play_sfx, Sounds},
    components::{Consumable, Defense, Spaceship},
    constants::ARENA_MIN_Y,
    systems::hitbox_collide,
};
use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Entities, Join, Read, ReadExpect, System, WriteStorage},
};
use std::ops::Deref;

pub struct ConsumableSystem;

impl<'s> System<'s> for ConsumableSystem {
    type SystemData = (
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

    fn run(
        &mut self,
        (
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
        for spaceship in (&mut spaceships).join() {
            for (consumable_entity, consumable, consumable_transform) in
                (&*entities, &mut consumables, &mut transforms).join()
            {
                let consumable_x = consumable_transform.translation().x;
                let consumable_y = consumable_transform.translation().y;

                if hitbox_collide(
                    consumable_x,
                    consumable_y,
                    spaceship.pos_x,
                    spaceship.pos_y,
                    consumable.hitbox_width,
                    consumable.hitbox_height,
                    spaceship.hitbox_width,
                    spaceship.hitbox_height,
                ) {
                    spaceship.health += consumable.health_value;
                    spaceship.money += consumable.money_value;

                    if consumable.money_value == 1 {
                        play_sfx(
                            &sounds.small_rock_sfx,
                            &storage,
                            audio_output.as_ref().map(|o| o.deref()),
                        );
                    } else if consumable.money_value == 5 {
                        play_sfx(
                            &sounds.large_rock_sfx,
                            &storage,
                            audio_output.as_ref().map(|o| o.deref()),
                        );
                    } else if consumable.health_value > 0.0 || consumable.defense_value > 0.0 {
                        play_sfx(
                            &sounds.wrench_sfx,
                            &storage,
                            audio_output.as_ref().map(|o| o.deref()),
                        );
                    }

                    for defense in (&mut defenses).join() {
                        defense.defense += consumable.defense_value;
                    }

                    let _result = entities.delete(consumable_entity);
                } else {
                    consumable_transform
                        .prepend_translation_y(-1.0 * consumable.speed * time.delta_seconds());

                    if consumable_transform.translation()[1] < ARENA_MIN_Y {
                        let _result = entities.delete(consumable_entity);
                    }
                }
            }
        }
    }
}
