use amethyst::{
    core::{
        transform::Transform,
        timing::Time,
    },
    ecs::prelude::{Entities, Join, System, ReadStorage, WriteStorage, Read, ReadExpect, LazyUpdate},
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
        ReadStorage<'s, Transform>,
        Read<'s, Time>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut items, mut spaceships, transforms, time, lazy_update): Self::SystemData) {
        for (spaceship, spaceship_transform) in (&mut spaceships, &transforms).join() {

            let spaceship_x = spaceship_transform.translation().x;
            let spaceship_y = spaceship_transform.translation().y;

            for (item_entity, item, item_transform) in (&*entities, &mut items, &transforms).join() {

                let item_x = item_transform.translation().x;
                let item_y = item_transform.translation().y;

                if hitbox_collide(item_x, item_y, spaceship_x, spaceship_y, item.hitbox_width, item.hitbox_height, spaceship.hitbox_width, spaceship.hitbox_height) {

                    //add stats to spaceship
                    if item.stat_effects.contains_key("barrel_damage") {
                        println!("stat effect added!");
                        spaceship.barrel_damage += item.stat_effects["barrel_damage"];
                    }

                    let _result = entities.delete(item_entity);
                }

            }


        }

    }
}

