use crate::{
    components::{EnemyComponent, EnemyType, Hitbox2DComponent, Motion2DComponent},
    constants::{ARENA_MAX_X, ARENA_MAX_Y, ARENA_MIN_X, ARENA_MIN_Y},
};
use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
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

            // limit speed x
            if motion_2d.velocity.x.abs() > motion_2d.max_speed.x {
                if motion_2d.velocity.x >= 0.0 {
                    motion_2d.velocity.x = motion_2d.max_speed.x;
                } else {
                    motion_2d.velocity.x = -motion_2d.max_speed.x;
                }
            }

            // limit speed y
            if motion_2d.velocity.y.abs() > motion_2d.max_speed.y {
                if motion_2d.velocity.y >= 0.0 {
                    motion_2d.velocity.y = motion_2d.max_speed.y;
                } else {
                    motion_2d.velocity.y = -motion_2d.max_speed.y;
                }
            }
        }
    }
}

pub struct EnemyMotion2DSystem;

impl<'s> System<'s> for EnemyMotion2DSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, EnemyComponent>,
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Hitbox2DComponent>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (entities, enemies, mut motion_2ds, mut transforms, hitbox_2ds, time): Self::SystemData,
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
            match enemy.enemy_type {
                EnemyType::Pawn => {
                    // accelerate in negative y direction
                    motion_2d.velocity.y += motion_2d.acceleration.y;
                }
                EnemyType::Drone => {
                    // accelerate in negative y direction
                    motion_2d.velocity.y += motion_2d.acceleration.y;
                }
                EnemyType::Hauler => {
                    // accelerate in negative y direction
                    motion_2d.velocity.y += motion_2d.acceleration.y;
                }
                EnemyType::Strafer => {
                    // TODO: Add motion logic
                    motion_2d.velocity.y += motion_2d.acceleration.y;
                    motion_2d.velocity.x += motion_2d.acceleration.x;
                }
                EnemyType::MissileLauncher => {
                    // TODO: Add motion logic
                    motion_2d.velocity.y += motion_2d.acceleration.y;
                }
                EnemyType::Missile => {
                    // TODO: Add motion logic
                    motion_2d.velocity.y += motion_2d.acceleration.y;
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

            // if enemy reaches the bottom despawn
            if transform.translation().y - hitbox_2d.height / 2.0 < ARENA_MIN_Y {
                entities
                    .delete(entity)
                    .expect("enemy unable to despawn upon reaching bottom of arena");
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
            /*
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
            */
        }
    }
}
