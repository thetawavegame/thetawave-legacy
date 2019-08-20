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

use crate::space_shooter::ARENA_MIN_Y;


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

            for (item_entity, item, item_transform) in (&*entities, &mut items, &mut transforms).join() {

                let item_x = item_transform.translation().x;
                let item_y = item_transform.translation().y;

                if hitbox_collide(item_x, item_y, spaceship.pos_x, spaceship.pos_y, item.hitbox_width, item.hitbox_height, spaceship.hitbox_width, spaceship.hitbox_height) {


                    //add stats to spaceship
                    if item.bool_effects.contains_key("barrel_immunity") {
                        spaceship.steel_barrel = item.bool_effects["barrel_immunity"];
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

                    if item.stat_effects.contains_key("acceleration") {
                        spaceship.acceleration_x += item.stat_effects["acceleration"];
                        spaceship.acceleration_y += item.stat_effects["acceleration"];
                    }

                    if item.stat_effects.contains_key("deceleration") {
                        spaceship.deceleration_x += item.stat_effects["deceleration"];
                        spaceship.deceleration_y += item.stat_effects["deceleration"];
                    }

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

