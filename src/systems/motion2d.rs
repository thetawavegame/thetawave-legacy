use crate::{
    components::{EnemyComponent, EnemyType, Hitbox2DComponent, Motion2DComponent},
    constants::{ARENA_MAX_X, ARENA_MAX_Y, ARENA_MIN_X, ARENA_MIN_Y},
};
use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

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

            // update translation
            transform.set_translation_xyz(
                transform.translation().x + motion_2d.velocity.x * dt,
                transform.translation().y + motion_2d.velocity.y * dt,
                transform.translation().z,
            );

            // update angular position
            transform.append_rotation_z_axis(motion_2d.angular_velocity * dt);
        }
    }
}

pub struct EnemyMotion2DSystem;

impl<'s> System<'s> for EnemyMotion2DSystem {
    type SystemData = (
        ReadStorage<'s, EnemyComponent>,
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (enemies, mut motion_2ds, mut transforms, time): Self::SystemData) {
        for (enemy, motion_2d, transform) in (&enemies, &mut motion_2ds, &mut transforms).join() {
            match enemy.enemy_type {
                EnemyType::Pawn => {
                    // accelerate in negative y direction
                    motion_2d.velocity.y -= motion_2d.acceleration.y;
                }
                EnemyType::Drone => {
                    // accelerate in negative y direction
                    motion_2d.velocity.y -= motion_2d.acceleration.y;
                }
                EnemyType::Hauler => {
                    // accelerate in negative y direction
                    motion_2d.velocity.y -= motion_2d.acceleration.y;
                }
                EnemyType::Strafer => {
                    // TODO: Add motion logic
                }
                EnemyType::MissileLauncher => {
                    // TODO: Add motion logic
                }
                EnemyType::Missile => {
                    // TODO: Add motion logic
                }
                EnemyType::RepeaterBody => {
                    // TODO: Add motion logic
                }
                EnemyType::RepeaterHead => {
                    // TODO: Add motion logic
                }
                EnemyType::RepeaterShoulder => {
                    // TODO: Add motion logic
                }
                EnemyType::RepeaterArm => {
                    // TODO: Add motion logic
                }
            }
        }
    }
}

pub struct ConstrainToArenaSystem;

impl<'s> System<'s> for ConstrainToArenaSystem {
    type SystemData = (
        ReadStorage<'s, Hitbox2DComponent>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Motion2DComponent>,
    );

    fn run(&mut self, (hitboxes, mut transforms, mut motions): Self::SystemData) {
        for (hitbox, transform, motion) in (&hitboxes, &mut transforms, &mut motions).join() {
            let x_pos = transform.translation().x - (hitbox.width / 2.0);
            let y_pos = transform.translation().y - (hitbox.height / 2.0);

            if x_pos < ARENA_MIN_X || x_pos > ARENA_MAX_X {
                motion.velocity.x *= -1.0;
                motion.acceleration.x *= -1.0;
            }

            if y_pos < ARENA_MIN_Y || y_pos > ARENA_MAX_Y {
                motion.velocity.y *= -1.0;
                motion.acceleration.y *= -1.0;
            }
        }
    }
}
