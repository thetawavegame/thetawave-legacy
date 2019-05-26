use amethyst::{
    core::{
        transform::Transform,
        timing::Time,
    },
    ecs::prelude::{Entities,Join, ReadStorage, System, WriteStorage, Read},
};

use crate::{
    components::{Spaceship, Enemy},
    systems::hitbox_collide,
};


pub struct BarrelRollSystem;
impl<'s> System<'s> for BarrelRollSystem {

    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Spaceship>,
        WriteStorage<'s, Enemy>,
        ReadStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, spaceships, mut enemies, transforms, time): Self::SystemData) {
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

