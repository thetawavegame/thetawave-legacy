use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities,Join, ReadStorage, System, WriteStorage},
};

use crate::components::{Spaceship, Enemy};

pub struct BarrelRollSystem;
impl<'s> System<'s> for BarrelRollSystem {

    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Spaceship>,
        WriteStorage<'s, Enemy>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, spaceships, mut enemies, transforms): Self::SystemData) {
        for (spaceship, spaceship_transform) in (&spaceships, &transforms).join() {

            let spaceship_x = spaceship_transform.translation().x;
            let spaceship_y = spaceship_transform.translation().y;

            for (enemy, enemy_transform) in (&mut enemies, &transforms).join() {

                let enemy_x = enemy_transform.translation().x;
                let enemy_y = enemy_transform.translation().y;


                if (spaceship.barrel_action_right || spaceship.barrel_action_left) && hitbox_collide(spaceship_x, spaceship_y, enemy_x, enemy_y, spaceship.hitbox_width, spaceship.hitbox_height, enemy.hitbox_width, enemy.hitbox_height) {
                    println!("collision");
                    enemy.health -= spaceship.barrel_damage;
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