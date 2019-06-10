use amethyst::{
    core::{
        transform::Transform,
        timing::Time,
        nalgebra::Vector3,
    },
    ecs::prelude::{Entities, Join, System, WriteStorage, Read, ReadExpect, LazyUpdate},
};

use crate::{
    components::{Enemy, DefenseBar},
    entities::spawn_explosion,
    resources::SpriteResource,
};
use crate::entities::fire_blast;
use crate::space_shooter::ARENA_MIN_Y;


pub struct EnemySystem;
impl<'s> System<'s> for EnemySystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, DefenseBar>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut enemys, mut defense_bars, mut transforms, time, sprite_resource, lazy_update): Self::SystemData) {
        for (enemy_entity, enemy_component, enemy_transform) in (&*entities, &mut enemys, &mut transforms).join() {

            let enemy_x = enemy_transform.translation().x;
            let enemy_y = enemy_transform.translation().y;

            //if current velocity is greater than the maximum knockback speed immediately set speed to the maximum knockback speed
            if enemy_component.current_velocity_x > enemy_component.knockback_max_speed {
                enemy_component.current_velocity_x = enemy_component.knockback_max_speed;
            }
            if enemy_component.current_velocity_x < ((-1.0)*enemy_component.knockback_max_speed) {
                enemy_component.current_velocity_x = (-1.0)*enemy_component.knockback_max_speed;
            }
            if enemy_component.current_velocity_y > enemy_component.knockback_max_speed {
                enemy_component.current_velocity_y = enemy_component.knockback_max_speed;
            }
            if enemy_component.current_velocity_y < ((-1.0)*enemy_component.knockback_max_speed) {
                enemy_component.current_velocity_y = (-1.0)*enemy_component.knockback_max_speed;
            }

            //if the current velocity is greater than the max velocity decelerate until at max velocity
            if enemy_component.current_velocity_y > enemy_component.max_speed {
                enemy_component.current_velocity_y -= enemy_component.deceleration_y;
            }
            if enemy_component.current_velocity_y < ((-1.0)*enemy_component.max_speed) {
                enemy_component.current_velocity_y += enemy_component.deceleration_y;
            }

            //if there any velocity in x direction decelerate in y and decelerate in x
            if enemy_component.current_velocity_x > 0.0 {
                if enemy_component.current_velocity_y < 0.0 {
                    enemy_component.current_velocity_y += enemy_component.deceleration_y;
                }
                enemy_component.current_velocity_x -= enemy_component.deceleration_x;
            }else if enemy_component.current_velocity_x < 0.0 {
                if enemy_component.current_velocity_y < 0.0 {
                    enemy_component.current_velocity_y += enemy_component.deceleration_y;
                }
                enemy_component.current_velocity_x += enemy_component.deceleration_x;
            }

            //accelerate in -y direction
            if enemy_component.current_velocity_y > (-1.0 * enemy_component.max_speed) {
                enemy_component.current_velocity_y -= enemy_component.acceleration_y;
            }

            //transform the spaceship in x and y by the currrent velocity in x and y
            enemy_transform.set_x(enemy_x + (enemy_component.current_velocity_x) * time.delta_seconds());
            enemy_transform.set_y(enemy_y + (enemy_component.current_velocity_y) * time.delta_seconds());

            //if the enemy can shoot
            if enemy_component.fires {

                //firing cooldown
                if enemy_component.fire_reset_timer > 0.0 {
                    enemy_component.fire_reset_timer -= time.delta_seconds();
                }else {
                    println!("enemy fire!");
                    let fire_position = Vector3::new(
                        enemy_transform.translation()[0], enemy_transform.translation()[1] - enemy_component.height / 2.0, 0.1,
                    );

                    fire_blast(&entities, &sprite_resource, 9, fire_position, enemy_component.blast_damage, 0.0, 0.0, enemy_component.blast_speed, false, &lazy_update);

                    enemy_component.fire_reset_timer = enemy_component.fire_speed;
                }
            }

            //spawn explosion if health is less than 0
            if enemy_component.health < 0.0 {

                let explosion_position = Vector3::new(
                    enemy_transform.translation()[0], enemy_transform.translation()[1], 0.0,
                );

                spawn_explosion(&entities, &sprite_resource, 4,explosion_position, &lazy_update);
            }

            //remove enemy and modify defense value if necessary
            if enemy_transform.translation()[1] < ARENA_MIN_Y || enemy_component.health < 0.0 {
                if enemy_transform.translation()[1] < ARENA_MIN_Y {
                    for defense_bar in (&mut defense_bars).join() {
                        defense_bar.defense -= enemy_component.defense_damage;
                    }

                }

                entities.delete(enemy_entity);

            }
        }
    }
}