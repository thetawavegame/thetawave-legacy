use amethyst::{
    core::{
        transform::Transform,
        timing::Time,
    },
    ecs::prelude::{Entities, Join, System, WriteStorage, Read},
};

use crate::{
    components::{Item, Spaceship},
    systems::hitbox_collide,
};


pub struct ItemSystem;
impl<'s> System<'s> for ItemSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Item>,
        WriteStorage<'s, Spaceship>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, mut items, mut spaceships, mut transforms, time): Self::SystemData) {
        for spaceship in (&mut spaceships).join() {

            //let spaceship_x = spaceship_transform.translation().x;
            //let spaceship_y = spaceship_transform.translation().y;

            for (item_entity, item, item_transform) in (&*entities, &mut items, &mut transforms).join() {

                let item_x = item_transform.translation().x;
                let item_y = item_transform.translation().y;

                if hitbox_collide(item_x, item_y, spaceship.pos_x, spaceship.pos_y, item.hitbox_width, item.hitbox_height, spaceship.hitbox_width, spaceship.hitbox_height) {


                    //add stats to spaceship
                    if item.stat_effects.contains_key("barrel_damage") {
                        spaceship.barrel_damage += item.stat_effects["barrel_damage"];
                    }

                    let _result = entities.delete(item_entity);
                }else {
                    item_transform.translate_y(-1.0 * item.speed * time.delta_seconds());
                }

            }


        }

    }
}

