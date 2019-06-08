use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities,Join, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{Spaceship, Enemy},
    systems::hitbox_collide,
};
use crate::space_shooter::{ARENA_MIN_X, ARENA_MAX_X, ARENA_MIN_Y, ARENA_MAX_Y};


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

            //spaceship collision with border
            if (spaceship_x - (spaceship.width/2.0)) < ARENA_MIN_X {
                if spaceship.barrel_action_left {
                    spaceship.barrel_action_right = true;
                    spaceship.barrel_action_left = false;
                }else {
                    spaceship.current_velocity_x = spaceship.current_velocity_x.abs();
                }
            }else if (spaceship_x + (spaceship.width/2.0)) > ARENA_MAX_X {
                if spaceship.barrel_action_right {
                    spaceship.barrel_action_left = true;
                    spaceship.barrel_action_right = false;
                }
                spaceship.current_velocity_x = (-1.0) * spaceship.current_velocity_x.abs();
            }else if (spaceship_y - (spaceship.height/2.0)) < ARENA_MIN_Y {
                spaceship.current_velocity_y = spaceship.current_velocity_y.abs();
            }else if (spaceship_y + (spaceship.height/2.0)) > ARENA_MAX_Y {
                spaceship.current_velocity_y = (-1.0) * spaceship.current_velocity_y.abs();
            }

            for (enemy, enemy_transform) in (&mut enemies, &transforms).join() {

                let enemy_x = enemy_transform.translation().x;
                let enemy_y = enemy_transform.translation().y;

                //enemy collision with border
                if (enemy_x - (enemy.width/2.0)) < ARENA_MIN_X || (enemy_x + (enemy.width/2.0)) > ARENA_MAX_X {
                    enemy.current_velocity_x = (-1.0) * enemy.current_velocity_x;
                }

                if hitbox_collide(enemy_x, enemy_y, spaceship_x, spaceship_y, enemy.hitbox_width, enemy.hitbox_height, spaceship.hitbox_width, spaceship.hitbox_height) {

                    enemy.health -= spaceship.collision_damage;

                    if spaceship.barrel_action_left {

                        if !spaceship.steel_barrel {
                            spaceship.health -= enemy.collision_damage;
                        }

                        spaceship.barrel_action_right = true;
                        spaceship.barrel_action_left = false;



                        let temp_velocity_y = spaceship.current_velocity_y;
                        spaceship.current_velocity_y += enemy.current_velocity_y;

                        enemy.current_velocity_x = (-(1.0) * enemy.current_velocity_x) - spaceship.barrel_speed;
                        enemy.current_velocity_y += temp_velocity_y;


                    }else if spaceship.barrel_action_right {

                        if !spaceship.steel_barrel {
                            spaceship.health -= enemy.collision_damage;
                        }

                        spaceship.barrel_action_left = true;
                        spaceship.barrel_action_right = false;

                        let temp_velocity_y = spaceship.current_velocity_y;
                        spaceship.current_velocity_y += enemy.current_velocity_y;

                        enemy.current_velocity_x = (-(1.0) * enemy.current_velocity_x) + spaceship.barrel_speed;
                        enemy.current_velocity_y += temp_velocity_y;

                    }else {
                        spaceship.health -= enemy.collision_damage;
                        let temp_velocity_x = spaceship.current_velocity_x;
                        spaceship.current_velocity_x = ((-(1.0) * spaceship.current_velocity_x) + enemy.current_velocity_x);
                        enemy.current_velocity_x = ((-(1.0) * enemy.current_velocity_x) + temp_velocity_x);

                        let temp_velocity_y = spaceship.current_velocity_y;
                        spaceship.current_velocity_y = ((-(1.0) * spaceship.current_velocity_y) + enemy.current_velocity_y);
                        enemy.current_velocity_y = ((-(1.0) * enemy.current_velocity_y) + temp_velocity_y);
                    }

                    println!("spaceship height: {}", (spaceship_y + (spaceship.hitbox_height/2.0)));
                    println!("enemy height: {}", (enemy_y));
                    if (spaceship_y + (spaceship.hitbox_height/2.0)) <= (enemy_y) {
                        if enemy.current_velocity_y < 100.0 {
                            enemy.current_velocity_y = 100.0;
                        }
                    }

                    if (spaceship_y - (spaceship.hitbox_height/2.0)) >= (enemy_y) {
                        if enemy.current_velocity_y < -100.0 {
                            enemy.current_velocity_y = -100.0;
                        }
                    }
                }
            }
        }
    }
}