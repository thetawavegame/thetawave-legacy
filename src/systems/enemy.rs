use crate::constants::ARENA_HEIGHT;
use crate::{
    components::{
        EnemyComponent, EnemyType, HealthComponent, Hitbox2DComponent, Motion2DComponent,
    },
    constants::ARENA_MIN_Y,
    events::{EnemyDestroyedEvent, EnemyReachedBottomEvent},
};
use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage},
    shrev::EventChannel,
};

pub struct EnemySystem;

impl<'s> System<'s> for EnemySystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, EnemyComponent>,
        WriteStorage<'s, HealthComponent>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Motion2DComponent>,
        ReadStorage<'s, Hitbox2DComponent>,
        Read<'s, Time>,
        Write<'s, EventChannel<EnemyDestroyedEvent>>,
        Write<'s, EventChannel<EnemyReachedBottomEvent>>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut enemies,
            mut healths,
            mut transforms,
            mut motions,
            hitboxes,
            time,
            mut enemy_destroyed_event_channel,
            mut enemy_reached_bottom_event_channel,
        ): Self::SystemData,
    ) {
        for (
            enemy_entity,
            enemy_component,
            enemy_health,
            enemy_transform,
            enemy_motion,
            enemy_hitbox,
        ) in (
            &*entities,
            &mut enemies,
            &mut healths,
            &mut transforms,
            &mut motions,
            &hitboxes,
        )
            .join()
        {
            // constrain in arena
            //enemy_component.constrain_to_arena(enemy_transform, enemy_motion, enemy_hitbox); // handled in constrain to arena system

            // transform the spaceship in x and y by the currrent velocity in x and y
            // enemy_component.update_position(enemy_transform, time.delta_seconds(), enemy_motion); // handle in motion2d system

            enemy_health.value -= enemy_component.poison;
            enemy_health.constrain();

            // conditions for despawning
            if enemy_transform.translation()[1] + enemy_hitbox.height / 2.0 < ARENA_MIN_Y {
                // defense is damaged if enemy gets past
                enemy_reached_bottom_event_channel
                    .single_write(EnemyReachedBottomEvent::new(enemy_component.defense_damage));
                entities
                    .delete(enemy_entity)
                    .expect("unable to delete entity");
            } else if enemy_health.value <= 0.0 {
                enemy_destroyed_event_channel.single_write(EnemyDestroyedEvent::new(enemy_entity));
            }

            // behavior for enemies based on its enemy_type attribute
            match enemy_component.enemy_type {
                EnemyType::Pawn => {
                    // accelerate in -y direction
                    //enemy_component.accelerate(0.0, -1.0, enemy_motion);
                }

                EnemyType::Drone => {
                    // accelerate in -y direction
                    // enemy_component.accelerate(0.0, -1.0, enemy_motion);
                }

                EnemyType::Hauler => {
                    // accelerate in -y direction
                    //enemy_component.accelerate(0.0, -1.0, enemy_motion);
                }

                EnemyType::Strafer => {
                    // accelerate in -y direction
                    //enemy_component.accelerate(0.0, -1.0, enemy_motion);
                    //enemy_component.accelerate(1.0, 0.0, enemy_motion);
                }

                EnemyType::MissileLauncher => {
                    //enemy_component.accelerate(0.0, -1.0, enemy_motion);
                }

                EnemyType::Missile => {
                    //enemy_component.accelerate(0.0, -1.0, enemy_motion);
                }

                EnemyType::RepeaterBody => {
                    // accelerate in -y direction
                    /*
                    if enemy_transform.translation().y > ARENA_MIN_Y + ARENA_HEIGHT - 30.0 {
                        enemy_component.accelerate(0.0, -1.0, enemy_motion);
                    } else {
                        enemy_motion.velocity.y = 0.0;
                    }
                    */
                }

                EnemyType::RepeaterHead => {
                    // accelerate in -y direction
                    /*
                    if enemy_transform.translation().y > ARENA_MIN_Y + ARENA_HEIGHT - 67.0 {
                        enemy_component.accelerate(0.0, -1.0, enemy_motion);
                    } else {
                        enemy_motion.velocity.y = 0.0;
                    }
                    */
                }

                EnemyType::RepeaterShoulder => {
                    // accelerate in -y direction
                    /*
                    if enemy_transform.translation().y > ARENA_MIN_Y + ARENA_HEIGHT - 32.0 {
                        enemy_component.accelerate(0.0, -1.0, enemy_motion);
                    } else {
                        enemy_motion.velocity.y = 0.0;
                    }

                    // rotate back and forth
                    if enemy_transform.euler_angles().2 > 0.1 {
                        enemy_motion.angular_velocity = 0.05;
                    } else if enemy_transform.euler_angles().2 < -0.1 {
                        enemy_motion.angular_velocity = -0.05;
                    }
                    */
                }

                EnemyType::RepeaterArm => {
                    /*
                    if enemy_transform.translation().y > ARENA_MIN_Y + ARENA_HEIGHT - 32.0 {
                        enemy_component.accelerate(0.0, -1.0, enemy_motion);
                    } else {
                        enemy_motion.velocity.y = 0.0;
                    }
                    */
                }
            }
        }
    }
}
