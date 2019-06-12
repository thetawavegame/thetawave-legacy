use amethyst::{
    core::{
        transform::Transform,
        timing::Time,
        nalgebra::Vector3,
    },
    ecs::prelude::{Entities, Join, System, WriteStorage, Read, ReadExpect, LazyUpdate},
};

use rand::{thread_rng, Rng};

use crate::{
    components::{Enemy, Defense, Rigidbody, Fires},
    entities::{spawn_explosion, spawn_consumable},
    resources::SpriteResource,
};
use crate::entities::fire_blast;
use crate::space_shooter::ARENA_MIN_Y;

const ENEMY_BLAST_SPRITE_INDEX: usize = 9;
const EXPLOSION_SPRITE_INDEX: usize = 4;

const EXPLOSION_Z: f32 = 0.0;


pub struct EnemySystem;
impl<'s> System<'s> for EnemySystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Defense>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut enemys, mut defenses, mut transforms, time, sprite_resource, lazy_update): Self::SystemData) {
        for (enemy_entity, enemy_component, enemy_transform) in (&*entities, &mut enemys, &mut transforms).join() {

            let enemy_x = enemy_transform.translation().x;
            let enemy_y = enemy_transform.translation().y;

            //limit the maximum knockback
            enemy_component.limit_knockback();

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
            enemy_component.accelerate(0.0, -1.0);

            //transform the spaceship in x and y by the currrent velocity in x and y
            enemy_component.update_position(enemy_transform, time.delta_seconds());

            //if the enemy can shoot
            if enemy_component.fires {

                if !enemy_component.fire_cooldown(time.delta_seconds()) {
                    let fire_position = Vector3::new(
                        enemy_transform.translation()[0], enemy_transform.translation()[1] - enemy_component.height / 2.0, 0.1,
                    );

                    fire_blast(&entities, &sprite_resource, ENEMY_BLAST_SPRITE_INDEX, fire_position, enemy_component.blast_damage, 0.0, 0.0, enemy_component.blast_speed, false, &lazy_update);

                    enemy_component.fire_reset_timer = enemy_component.fire_speed;
                }
            }

            //spawn explosion if health is less than 0
            if enemy_component.health < 0.0 {

                let death_position = Vector3::new(
                    enemy_transform.translation()[0], enemy_transform.translation()[1], EXPLOSION_Z,
                );

                spawn_explosion(&entities, &sprite_resource, EXPLOSION_SPRITE_INDEX,death_position, &lazy_update);

                //chance to spawn consumable
                if thread_rng().gen::<f32>() < enemy_component.drop_chance {
                    spawn_consumable(&entities, &sprite_resource, &mut enemy_component.consumable_pool, death_position, &lazy_update);
                }
            }

            //remove enemy and modify defense value if necessary
            if enemy_transform.translation()[1] < ARENA_MIN_Y || enemy_component.health < 0.0 {
                if enemy_transform.translation()[1] < ARENA_MIN_Y {
                    for defense in (&mut defenses).join() {
                        defense.defense -= enemy_component.defense_damage;
                    }

                }

                let _result = entities.delete(enemy_entity);

            }
        }
    }
}