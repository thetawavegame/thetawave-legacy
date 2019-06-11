use amethyst::{
    core::{
        transform::Transform,
        timing::Time,
    },
    ecs::prelude::{Entities, Join, System, WriteStorage, Read},
};

use crate::{
    components::{Consumable, Spaceship, DefenseBar},
    systems::hitbox_collide,
};

use crate::space_shooter::ARENA_MIN_Y;


pub struct ConsumableSystem;
impl<'s> System<'s> for ConsumableSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Consumable>,
        WriteStorage<'s, Spaceship>,
        WriteStorage<'s, DefenseBar>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, mut consumables, mut spaceships, mut defense_bars, mut transforms, time): Self::SystemData) {
        for spaceship in (&mut spaceships).join() {

            for (consumable_entity, consumable, consumable_transform) in (&*entities, &mut consumables, &mut transforms).join() {

                let consumable_x = consumable_transform.translation().x;
                let consumable_y = consumable_transform.translation().y;

                if hitbox_collide(consumable_x, consumable_y, spaceship.pos_x, spaceship.pos_y, consumable.hitbox_width, consumable.hitbox_height, spaceship.hitbox_width, spaceship.hitbox_height) {

                    spaceship.health += consumable.health_value;

                    for defense_bar in (&mut defense_bars).join() {
                        defense_bar.defense += consumable.defense_value;
                    }


                    let _result = entities.delete(consumable_entity);
                }else {
                    consumable_transform.translate_y(-1.0 * consumable.speed * time.delta_seconds());

                    if consumable_transform.translation()[1] < ARENA_MIN_Y {
                        let _result = entities.delete(consumable_entity);
                    }
                }

            }


        }

    }
}
