use crate::{
    components::{EnemyComponent, EnemyType, Hitbox2DComponent, Motion2DComponent},
    constants::{ARENA_HEIGHT, ARENA_MAX_X, ARENA_MIN_X, ARENA_MIN_Y},
};
use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
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
                if motion_2d.velocity.x >= 0.0 {
                    motion_2d.velocity.x = motion_2d.max_speed.x;
                } else {
                    motion_2d.velocity.x = -motion_2d.max_speed.x;
                }
            }

            // limit speed in the y direction to the max speed
            if motion_2d.velocity.y.abs() > motion_2d.max_speed.y {
                if motion_2d.velocity.y >= 0.0 {
                    motion_2d.velocity.y = motion_2d.max_speed.y;
                } else {
                    motion_2d.velocity.y = -motion_2d.max_speed.y;
                }
            }

            // constrain motion2d entities to area
        }
    }
}

// motion behavior for enemies
pub struct EnemyMotion2DSystem;

impl<'s> System<'s> for EnemyMotion2DSystem {
    type SystemData = (
        ReadStorage<'s, EnemyComponent>,
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Hitbox2DComponent>,
    );

    fn run(&mut self, (enemies, mut motion_2ds, mut transforms, hitbox_2ds): Self::SystemData) {
        for (enemy, motion_2d, hitbox_2d, transform) in
            (&enemies, &mut motion_2ds, &hitbox_2ds, &mut transforms).join()
        {
            move_enemy(&enemy, transform, motion_2d);
            constrain_enemies_to_arena(&enemy, transform, motion_2d, hitbox_2d);
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
            // accelerate in negative y direction
            motion_2d.velocity.y += motion_2d.acceleration.y;
            // accelerate in x direction
            motion_2d.velocity.x += motion_2d.acceleration.x;
        }
        EnemyType::MissileLauncher => {
            // accelerate in negative y direction
            motion_2d.velocity.y += motion_2d.acceleration.y;
        }
        EnemyType::Missile => {
            // accelerate in negative y direction
            motion_2d.velocity.y += motion_2d.acceleration.y;
        }
        EnemyType::RepeaterBody => {
            // move down to position and then accelerate backwards
            if transform.translation().y > ARENA_MIN_Y + ARENA_HEIGHT - 30.0 {
                motion_2d.velocity.y += motion_2d.acceleration.y;
            } else {
                motion_2d.velocity.y -= motion_2d.acceleration.y;
            }
        }
        EnemyType::RepeaterHead => {
            // move down to position and then accelerate backwards
            if transform.translation().y > ARENA_MIN_Y + ARENA_HEIGHT - 67.0 {
                motion_2d.velocity.y += motion_2d.acceleration.y;
            } else {
                motion_2d.velocity.y -= motion_2d.acceleration.y;
            }
        }
        EnemyType::RepeaterShoulder => {
            // move down to position and then accelerate backwards
            if transform.translation().y > ARENA_MIN_Y + ARENA_HEIGHT - 32.0 {
                motion_2d.velocity.y += motion_2d.acceleration.y;
            } else {
                motion_2d.velocity.y -= motion_2d.acceleration.y;
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
                motion_2d.velocity.y += motion_2d.acceleration.y;
            } else {
                motion_2d.velocity.y -= motion_2d.acceleration.y;
            }
        }
    }
}

// how enemies behave upon coming into contact with the edges of the arena
fn constrain_enemies_to_arena(
    enemy: &EnemyComponent,
    transform: &mut Transform,
    motion_2d: &mut Motion2DComponent,
    hitbox_2d: &Hitbox2DComponent,
) {
    // right and left sides
    if transform.translation().x + (hitbox_2d.width / 2.0) > ARENA_MAX_X
        || transform.translation().x - (hitbox_2d.width / 2.0) < ARENA_MIN_X
    {
        match enemy.enemy_type {
            EnemyType::Strafer => {
                motion_2d.acceleration.x *= -1.0;
                motion_2d.velocity.x *= -1.0;
            }
            _ => {
                motion_2d.velocity.x *= -1.0;
            }
        }
    }
}
