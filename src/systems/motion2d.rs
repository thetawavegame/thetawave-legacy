use crate::{
    components::{EnemyComponent, EnemyType, Hitbox2DComponent, Motion2DComponent},
    constants::{ARENA_HEIGHT, ARENA_MAX_X, ARENA_MIN_X, ARENA_MIN_Y},
    events::EnemyReachedBottomEvent,
};
use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Entities, Entity, Join, Read, ReadStorage, System, Write, WriteStorage},
    shrev::EventChannel,
};

// basic physics for all Motion2D entities
pub struct Motion2DSystem;

impl<'s> System<'s> for Motion2DSystem {
    type SystemData = (
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut motion_2ds, mut transforms, time): Self::SystemData) {
        for (motion_2d, transform) in (&mut motion_2ds, &mut transforms).join() {
            let dt = time.delta_seconds();

            // update translation based on velocity and delta time
            transform.set_translation_xyz(
                transform.translation().x + motion_2d.velocity.x * dt,
                transform.translation().y + motion_2d.velocity.y * dt,
                transform.translation().z,
            );

            // update angle based on angular velocity and time
            transform.append_rotation_z_axis(motion_2d.angular_velocity * dt);

            // limit speed in the x direction to the max speed
            if motion_2d.velocity.x.abs() > motion_2d.max_speed.x {
                if motion_2d.velocity.x > 0.0 {
                    motion_2d.velocity.x = motion_2d.max_speed.x;
                } else {
                    motion_2d.velocity.x = -motion_2d.max_speed.x;
                }
            }

            // limit speed in the y direction to the max speed
            if motion_2d.velocity.y.abs() > motion_2d.max_speed.y {
                if motion_2d.velocity.y > 0.0 {
                    motion_2d.velocity.y = motion_2d.max_speed.y;
                } else {
                    motion_2d.velocity.y = -motion_2d.max_speed.y;
                }
            }
        }
    }
}

// motion behavior for enemies
pub struct EnemyMotion2DSystem;

impl<'s> System<'s> for EnemyMotion2DSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, EnemyComponent>,
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Hitbox2DComponent>,
        Write<'s, EventChannel<EnemyReachedBottomEvent>>,
    );

    fn run(
        &mut self,
        (
            entities,
            enemies,
            mut motion_2ds,
            mut transforms,
            hitbox_2ds,
            mut enemy_reached_bottom_event_channel,
        ): Self::SystemData,
    ) {
        for (enemy, motion_2d, hitbox_2d, transform, entity) in (
            &enemies,
            &mut motion_2ds,
            &hitbox_2ds,
            &mut transforms,
            &entities,
        )
            .join()
        {
            move_enemy(&enemy, transform, motion_2d);

            constrain_enemies_to_arena(
                &enemy,
                transform,
                motion_2d,
                hitbox_2d,
                &entity,
                &mut enemy_reached_bottom_event_channel,
                &entities,
            );
        }
    }
}

fn move_enemy(
    enemy: &EnemyComponent,
    transform: &mut Transform,
    motion_2d: &mut Motion2DComponent,
) {
    match enemy.enemy_type {
        EnemyType::Pawn => {
            motion_2d.move_down();
            motion_2d.brake_horizontal();
        }
        EnemyType::Drone => {
            motion_2d.move_down();
            motion_2d.brake_horizontal();
        }
        EnemyType::Hauler => {
            motion_2d.move_down();
            motion_2d.brake_horizontal();
        }
        EnemyType::Strafer => {
            motion_2d.move_down();

            // accelerate to speed stat in the x direction
            if motion_2d.velocity.x.abs() < motion_2d.speed.x {
                if motion_2d.velocity.x >= 0.0 {
                    motion_2d.velocity.x += motion_2d.acceleration.x;
                } else {
                    motion_2d.velocity.x -= motion_2d.acceleration.x;
                }
            } else if motion_2d.velocity.x.abs() >= motion_2d.speed.x {
                if motion_2d.velocity.x > 0.0 {
                    motion_2d.velocity.x -= motion_2d.deceleration.x;
                } else {
                    motion_2d.velocity.x += motion_2d.deceleration.x;
                }
            }
        }
        EnemyType::MissileLauncher => {
            motion_2d.move_down();
            motion_2d.brake_horizontal();
        }
        EnemyType::Missile => {
            motion_2d.move_down();
            motion_2d.brake_horizontal();
        }
        EnemyType::RepeaterBody => {
            // move down to position and then accelerate backwards
            if transform.translation().y > ARENA_MIN_Y + ARENA_HEIGHT - 30.0 {
                motion_2d.move_down();
            } else {
                motion_2d.move_up();
            }
        }
        EnemyType::RepeaterHead => {
            // move down to position and then accelerate backwards
            if transform.translation().y > ARENA_MIN_Y + ARENA_HEIGHT - 67.0 {
                motion_2d.move_down();
            } else {
                motion_2d.move_up();
            }
        }
        EnemyType::RepeaterShoulder => {
            // move down to position and then accelerate backwards
            if transform.translation().y > ARENA_MIN_Y + ARENA_HEIGHT - 32.0 {
                motion_2d.move_down();
            } else {
                motion_2d.move_up();
            }

            // rotate back and forth
            if transform.euler_angles().2 > 0.1 {
                motion_2d.angular_velocity = 0.05;
            } else if transform.euler_angles().2 < -0.1 {
                motion_2d.angular_velocity = -0.05;
            }
        }
        EnemyType::RepeaterArm => {
            // move down to position and then accelerate backwards
            if transform.translation().y > ARENA_MIN_Y + ARENA_HEIGHT - 32.0 {
                motion_2d.move_down();
            } else {
                motion_2d.move_up();
            }
        }
    }
}

// how enemies behave upon coming into contact with the edge of the arena
fn constrain_enemies_to_arena(
    enemy: &EnemyComponent,
    transform: &mut Transform,
    motion_2d: &mut Motion2DComponent,
    hitbox_2d: &Hitbox2DComponent,
    entity: &Entity,
    enemy_reached_bottom_event_channel: &mut Write<EventChannel<EnemyReachedBottomEvent>>,
    entities: &Entities,
) {
    // right and left sides
    if transform.translation().x + (hitbox_2d.width / 2.0) > ARENA_MAX_X
        || transform.translation().x - (hitbox_2d.width / 2.0) < ARENA_MIN_X
    {
        motion_2d.velocity.x *= -1.0;
    }

    // all enemies despawn upon reaching bottom side
    if transform.translation().y - hitbox_2d.height / 2.0 < ARENA_MIN_Y {
        enemy_reached_bottom_event_channel
            .single_write(EnemyReachedBottomEvent::new(enemy.defense_damage));
        entities
            .delete(*entity)
            .expect("enemy unable to despawn upon reaching bottom of arena");
    }
}
