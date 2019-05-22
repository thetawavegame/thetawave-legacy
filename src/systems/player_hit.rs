use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities,Join, ReadStorage, System, WriteStorage},
};

use crate::components::{Enemy, Blast};

pub struct PlayerHitSystem;
impl<'s> System<'s> for PlayerHitSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Blast>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, mut enemies, mut blasts, transforms): Self::SystemData) {
        for (enemy, enemy_transform) in (&mut enemies, &transforms).join() {

            let enemy_x = enemy_transform.translation().x;
            let enemy_y = enemy_transform.translation().y;

            for (blast_entity, blast, blast_transform) in (&*entities, &mut blasts, &transforms).join() {

                let blast_x = blast_transform.translation().x;
                let blast_y = blast_transform.translation().y;

                //detect if blast in box
                /*
                if point_in_rect(
                    blast_x,
                    blast_y,
                    enemy_x - (enemy.hitbox_width / 2.0),
                    enemy_y - (enemy.hitbox_height / 2.0),
                    enemy_x + (enemy.hitbox_width / 2.0),
                    enemy_y + (enemy.hitbox_height / 2.0),
                ){
                    blast.collision = true;
                }
                */

                if hitbox_collide(blast_x, blast_y, enemy_x, enemy_y, blast.hitbox_radius, blast.hitbox_radius, enemy.hitbox_width, enemy.hitbox_height) {
                    blast.collision = true;
                    let _result = entities.delete(blast_entity);
                    enemy.health -= blast.damage;
                }

            }


        }
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}

fn hitbox_collide(mut x1: f32, mut y1: f32, mut x2: f32, mut y2: f32, hitbox_width_1: f32, hitbox_height_1: f32, hitbox_width_2: f32, hitbox_height_2: f32) -> bool {

    x1 -= hitbox_width_1 / 2.0;
    y1 -= hitbox_height_1 / 2.0;

    x2 -= hitbox_width_2 / 2.0;
    y2 -= hitbox_height_2 / 2.0 ;


    x1 < (x2 + hitbox_width_2) && (x1 + hitbox_width_1) > x2 && y1 < (y2 + hitbox_height_2) && (y1 + hitbox_height_1) > y2
    //(x1 + hitbox_width_1 / 2.0) > (x2 - hitbox_width_2 / 2.0) && (x1 - hitbox_width_1 / 2.0) < (x2 + hitbox_width_2 / 2.0) && (y1 - hitbox_height_1 / 2.0) > (y2 + hitbox_height_2 / 2.0) && (y1 + hitbox_height_1 / 2.0) < (y2 - hitbox_height_2)
}
