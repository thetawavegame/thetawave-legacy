use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{Spaceship, Enemy},
    systems::hitbox_collide,
};


pub struct SpaceshipEnemyCollisionSystem;
impl<'s> System<'s> for SpaceshipEnemyCollisionSystem {

    type SystemData = (
        WriteStorage<'s, Spaceship>,
        WriteStorage<'s, Enemy>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut spaceships, mut enemies, transforms): Self::SystemData) {
        for (spaceship, spaceship_transform) in (&mut spaceships, &transforms).join() {

            let spaceship_x = spaceship_transform.translation().x;
            let spaceship_y = spaceship_transform.translation().y;

            for (enemy, enemy_transform) in (&mut enemies, &transforms).join() {

                let enemy_x = enemy_transform.translation().x;
                let enemy_y = enemy_transform.translation().y;

                if hitbox_collide(enemy_x, enemy_y, spaceship_x, spaceship_y, enemy.hitbox_width, enemy.hitbox_height, spaceship.hitbox_width, spaceship.hitbox_height) {
                    let mut enemy_dead = false;
                    if enemy.health <= 0.0 {
                        enemy_dead = true;
                    }

                    enemy.health -= spaceship.collision_damage;

                    if spaceship.barrel_action_left {
                        spaceship.barrel_action_right = true;
                        spaceship.barrel_action_left = false;

                    }else if spaceship.barrel_action_right {
                        spaceship.barrel_action_left = true;
                        spaceship.barrel_action_right = false;

                    }

                    if !spaceship.steel_barrel && !enemy_dead {
                        spaceship.health -= enemy.collision_damage;
                    }else if !spaceship.barrel_action_left && !spaceship.barrel_action_right{
                        spaceship.health -= enemy.collision_damage;
                    }

                    let temp_velocity_x = spaceship.current_velocity_x;
                    spaceship.current_velocity_x = (-(1.0) * spaceship.current_velocity_x) + enemy.current_velocity_x;
                    enemy.current_velocity_x = (-(1.0) * enemy.current_velocity_x) + temp_velocity_x;

                    let temp_velocity_y = spaceship.current_velocity_y;
                    spaceship.current_velocity_y = (-(1.0) * spaceship.current_velocity_y) + enemy.current_velocity_y;
                    enemy.current_velocity_y = (-(1.0) * enemy.current_velocity_y) + temp_velocity_y;
                }
            }
        }
    }
}