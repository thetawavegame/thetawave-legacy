use crate::{
    components::{
        ConsumableComponent, Hitbox2DComponent, MobComponent, Motion2DComponent, PlayerComponent,
    },
    constants::{ARENA_HEIGHT, ARENA_MIN_Y},
    entities::{AllyType, EnemyType, MobType, SpawnableCategory, SpawnableType},
    events::AttractionEvent,
};
use amethyst::{
    core::{math::Vector2, timing::Time, transform::Transform},
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
    ecs::*,
    shrev::{EventChannel, ReaderId},
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

// motion behavior for consumables
#[derive(Default)]
pub struct ConsumableMotion2DSystem {
    event_reader: Option<ReaderId<AttractionEvent>>,
}

impl<'s> System<'s> for ConsumableMotion2DSystem {
    type SystemData = (
        WriteStorage<'s, Motion2DComponent>,
        ReadStorage<'s, ConsumableComponent>,
        WriteStorage<'s, Transform>,
        Read<'s, EventChannel<AttractionEvent>>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<AttractionEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (mut motion_2ds, consumables, mut transforms, attraction_channel): Self::SystemData,
    ) {
        for event in attraction_channel.read(self.event_reader.as_mut().unwrap()) {
            for (_consumable, motion_2d, transform) in
                (&consumables, &mut motion_2ds, &mut transforms).join()
            {
                // check if spawnable is in area of influence
                if get_distance(
                    transform.translation().x,
                    event.target_position.x,
                    transform.translation().y,
                    event.target_position.y,
                ) < event.radius
                {
                    for spawnable_category in event.affected_spawnables.iter() {
                        if let SpawnableCategory::Consumable = spawnable_category {
                            // accelerate towards attractor
                            println!("attracting");
                            motion_2d.target_position = Some(event.target_position);
                            motion_2d.acceleration.x = event.acceleration;
                            motion_2d.acceleration.y = event.acceleration;
                            motion_2d.move_towards_target(Vector2::new(
                                transform.translation().x,
                                transform.translation().y,
                            ));
                            break;
                        }
                    }
                }
            }
        }
    }
}

// motion behavior for enemies
pub struct MobMotion2DSystem;

impl<'s> System<'s> for MobMotion2DSystem {
    type SystemData = (
        ReadStorage<'s, MobComponent>,
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Hitbox2DComponent>,
    );

    fn run(&mut self, (mobs, mut motion_2ds, mut transforms, mut hitbox_2ds): Self::SystemData) {
        for (mob, motion_2d, hitbox_2d, transform) in
            (&mobs, &mut motion_2ds, &mut hitbox_2ds, &mut transforms).join()
        {
            move_mob(&mob, transform, motion_2d, hitbox_2d);
        }
    }
}

// acquire target for targeting enemies
pub struct MobTargetSystem;

impl<'s> System<'s> for MobTargetSystem {
    type SystemData = (
        WriteStorage<'s, MobComponent>,
        WriteStorage<'s, Motion2DComponent>,
        ReadStorage<'s, PlayerComponent>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut mobs, mut motion_2ds, players, transforms): Self::SystemData) {
        for (mob, transform, motion_2d) in (&mut mobs, &transforms, &mut motion_2ds).join() {
            if let SpawnableType::Mob(MobType::Enemy(EnemyType::Missile)) = mob.spawnable_type {
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

                motion_2d.target_position = closest_player_position;
            }
        }
    }
}

fn move_mob(
    mob: &MobComponent,
    transform: &mut Transform,
    motion_2d: &mut Motion2DComponent,
    hitbox_2d: &mut Hitbox2DComponent,
) {
    match mob.spawnable_type {
        SpawnableType::Mob(MobType::Enemy(EnemyType::Pawn)) => {
            motion_2d.move_down();
            motion_2d.brake_horizontal();
        }
        SpawnableType::Mob(MobType::Enemy(EnemyType::Drone)) => {
            motion_2d.move_down();
            motion_2d.brake_horizontal();
        }
        SpawnableType::Mob(MobType::Ally(AllyType::Hauler)) => {
            motion_2d.move_down();
            motion_2d.brake_horizontal();
        }
        SpawnableType::Mob(MobType::Enemy(EnemyType::StraferRight)) => {
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
        SpawnableType::Mob(MobType::Enemy(EnemyType::StraferLeft)) => {
            motion_2d.move_down();

            // accelerate to speed stat in the x direction
            if motion_2d.velocity.x.abs() < motion_2d.speed.x {
                if motion_2d.velocity.x <= 0.0 {
                    motion_2d.velocity.x -= motion_2d.acceleration.x;
                } else {
                    motion_2d.velocity.x += motion_2d.acceleration.x;
                }
            } else if motion_2d.velocity.x.abs() >= motion_2d.speed.x {
                if motion_2d.velocity.x > 0.0 {
                    motion_2d.velocity.x -= motion_2d.deceleration.x;
                } else {
                    motion_2d.velocity.x += motion_2d.deceleration.x;
                }
            }
        }
        SpawnableType::Mob(MobType::Enemy(EnemyType::MissileLauncher)) => {
            motion_2d.move_down();
            motion_2d.brake_horizontal();
        }
        SpawnableType::Mob(MobType::Enemy(EnemyType::Missile)) => {
            if motion_2d.target_position.is_some() {
                //turn towards target
                motion_2d.turn_towards_target(
                    Vector2::new(transform.translation().x, transform.translation().y),
                    transform.euler_angles().2.to_degrees() + 180.0,
                );
                hitbox_2d.set_offset_rotation(transform.euler_angles().2);

                motion_2d.move_forward(transform.euler_angles().2);
            } else {
                motion_2d.move_down();
                motion_2d.brake_horizontal();
            }
        }
        SpawnableType::Mob(MobType::Enemy(EnemyType::RepeaterBody)) => {
            // move down to position and then accelerate backwards
            if transform.translation().y > ARENA_MIN_Y + ARENA_HEIGHT - 30.0 {
                motion_2d.move_down();
            } else {
                motion_2d.move_up();
            }
        }
        SpawnableType::Mob(MobType::Enemy(EnemyType::RepeaterHead)) => {
            // move down to position and then accelerate backwards
            if transform.translation().y > ARENA_MIN_Y + ARENA_HEIGHT - 67.0 {
                motion_2d.move_down();
            } else {
                motion_2d.move_up();
            }
        }
        SpawnableType::Mob(MobType::Enemy(EnemyType::RepeaterRightShoulder))
        | SpawnableType::Mob(MobType::Enemy(EnemyType::RepeaterLeftShoulder)) => {
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
        SpawnableType::Mob(MobType::Enemy(EnemyType::RepeaterRightArm))
        | SpawnableType::Mob(MobType::Enemy(EnemyType::RepeaterLeftArm)) => {
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
