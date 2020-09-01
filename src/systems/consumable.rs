use crate::{
    audio::{play_sfx, Sounds},
    components::{Consumable, Defense, Hitbox2DComponent, Spaceship},
    constants::ARENA_MIN_Y,
    systems::hitbox_collide,
};
use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Entities, Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
};

pub struct ConsumableSystem;

impl<'s> System<'s> for ConsumableSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Consumable>,
        WriteStorage<'s, Spaceship>,
        ReadStorage<'s, Hitbox2DComponent>,
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
            hitboxes,
            mut defenses,
            mut transforms,
            time,
            storage,
            sounds,
            audio_output,
        ): Self::SystemData,
    ) {
        for (spaceship, spaceship_hitbox) in (&mut spaceships, &hitboxes).join() {
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
                    spaceship_hitbox.width,
                    spaceship_hitbox.height,
                    consumable.hitbox_x_offset,
                    consumable.hitbox_y_offset,
                    spaceship_hitbox.offset_x,
                    spaceship_hitbox.offset_y,
                ) {
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
