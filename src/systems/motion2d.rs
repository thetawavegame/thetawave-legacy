use crate::{
    components::{EnemyComponent, Hitbox2DComponent, Motion2DComponent, PlayerComponent},
    constants::{ARENA_HEIGHT, ARENA_MIN_Y},
    entities::EntityType,
};
use amethyst::{
    core::{math::Vector2, timing::Time, transform::Transform},
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

            // limit angular speed to max angular speed
            if motion_2d.angular_velocity.abs() > motion_2d.angular_speed {
                if motion_2d.angular_velocity > 0.0 {
                    motion_2d.angular_velocity = motion_2d.angular_speed;
                } else {
                    motion_2d.angular_velocity = -motion_2d.angular_speed;
                }
            }

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
        ReadStorage<'s, EnemyComponent>,
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Hitbox2DComponent>,
    );

    fn run(&mut self, (enemies, mut motion_2ds, mut transforms, mut hitbox_2ds): Self::SystemData) {
        for (enemy, motion_2d, hitbox_2d, transform) in
            (&enemies, &mut motion_2ds, &mut hitbox_2ds, &mut transforms).join()
        {
            move_enemy(&enemy, transform, motion_2d, hitbox_2d);
        }
    }
}

// acquire target for targeting enemies
pub struct EnemyTargetSystem;

impl<'s> System<'s> for EnemyTargetSystem {
    type SystemData = (
        WriteStorage<'s, EnemyComponent>,
        ReadStorage<'s, PlayerComponent>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut enemies, players, transforms): Self::SystemData) {
        for (enemy, transform) in (&mut enemies, &transforms).join() {
            if let EntityType::Missile = enemy.entity_type {
                let mut closest_player_position: Option<Vector2<f32>> = None;

                for (_player, player_transform) in (&players, &transforms).join() {
                    if let Some(closest_position) = closest_player_position {
                        if get_distance(
                            player_transform.translation().x,
                            transform.translation().x,
                            player_transform.translation().y,
                            transform.translation().y,
                        ) < get_distance(
                            closest_position.x,
                            transform.translation().x,
                            closest_position.y,
                            transform.translation().y,
                        ) {
                            closest_player_position = Some(Vector2::new(
                                player_transform.translation().x,
                                player_transform.translation().y,
                            ));
                        }
                    } else {
                        closest_player_position = Some(Vector2::new(
                            player_transform.translation().x,
                            player_transform.translation().y,
                        ));
                    }
                }

                enemy.target_position = closest_player_position;
            }
        }
    }
}

fn move_enemy(
    enemy: &EnemyComponent,
    transform: &mut Transform,
    motion_2d: &mut Motion2DComponent,
    hitbox_2d: &mut Hitbox2DComponent,
) {
    match enemy.entity_type {
        EntityType::Pawn => {
            motion_2d.move_down();
            motion_2d.brake_horizontal();
        }
        EntityType::Drone => {
            motion_2d.move_down();
            motion_2d.brake_horizontal();
        }
        EntityType::Hauler => {
            motion_2d.move_down();
            motion_2d.brake_horizontal();
        }
        EntityType::Strafer => {
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
        EntityType::MissileLauncher => {
            motion_2d.move_down();
            motion_2d.brake_horizontal();
        }
        EntityType::Missile => {
            if let Some(target_position) = enemy.target_position {
                //turn towards target
                motion_2d.turn_towards_target(
                    Vector2::new(transform.translation().x, transform.translation().y),
                    transform.euler_angles().2.to_degrees() + 180.0,
                    target_position,
                );
                hitbox_2d.set_offset_rotation(transform.euler_angles().2);

                motion_2d.move_forward(transform.euler_angles().2);
            } else {
                motion_2d.move_down();
                motion_2d.brake_horizontal();
            }
        }
        EntityType::RepeaterBody => {
            // move down to position and then accelerate backwards
            if transform.translation().y > ARENA_MIN_Y + ARENA_HEIGHT - 30.0 {
                motion_2d.move_down();
            } else {
                motion_2d.move_up();
            }
        }
        EntityType::RepeaterHead => {
            // move down to position and then accelerate backwards
            if transform.translation().y > ARENA_MIN_Y + ARENA_HEIGHT - 67.0 {
                motion_2d.move_down();
            } else {
                motion_2d.move_up();
            }
        }
        EntityType::RepeaterRightShoulder | EntityType::RepeaterLeftShoulder => {
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
        EntityType::RepeaterRightArm | EntityType::RepeaterLeftArm => {
            // move down to position and then accelerate backwards
            if transform.translation().y > ARENA_MIN_Y + ARENA_HEIGHT - 32.0 {
                motion_2d.move_down();
            } else {
                motion_2d.move_up();
            }
        }

        _ => {}
    }
}

fn get_distance(x1: f32, x2: f32, y1: f32, y2: f32) -> f32 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}
