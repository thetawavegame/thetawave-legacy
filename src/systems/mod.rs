use crate::components::{BarrierComponent, Motion2DComponent, PushDirection};
use amethyst::core::math::Vector2;

mod ability;
mod animation;
mod autofire_system;
mod boss;
mod child_spawner;
mod collision_detection;
mod defense;
mod despawn;
mod enemy_behavior;
mod enemy_collisions;
mod enemy_destroyed;
mod fade;
mod gamemaster;
mod manualblaster_system;
mod motion2d;
mod planets;
mod play_audio;
mod spaceship;
mod spaceship_collisions;
mod spaceship_movement;
mod spawner;
mod stat_tracker;
mod status_bar;
mod store;
mod timelimit;

pub use self::{
    ability::BarrelRollAbilitySystem,
    animation::AnimationSystem,
    autofire_system::AutoFireSystem,
    boss::BossSystem,
    child_spawner::AutoSpawnerSystem,
    collision_detection::{CollisionDetectionSystem, CollisionHandlerSystem},
    defense::DefenseSystem,
    despawn::DespawnAtBorderSystem,
    enemy_behavior::EnemyBehaviorSystem,
    enemy_collisions::{
        EnemyArenaBorderCollisionSystem, EnemyBlastCollisionSystem, EnemyEnemyCollisionSystem,
        EnemyPlayerCollisionSystem,
    },
    enemy_destroyed::EnemyDestroyedSystem,
    fade::FadeSystem,
    gamemaster::PhaseManagerSystem,
    manualblaster_system::ManualBlasterSystem,
    motion2d::{EnemyMotion2DSystem, EnemyTargetSystem, Motion2DSystem},
    planets::PlanetsSystem,
    play_audio::PlayAudioSystem,
    spaceship::SpaceshipSystem,
    spaceship_collisions::{
        SpaceshipArenaBorderCollisionSystem, SpaceshipBlastCollisionSystem,
        SpaceshipConsumableCollisionSystem, SpaceshipEnemyCollisionSystem,
        SpaceshipItemCollisionSystem,
    },
    spaceship_movement::SpaceshipMovementSystem,
    spawner::SpawnerSystem,
    stat_tracker::StatTrackerSystem,
    status_bar::StatusBarSystem,
    store::StoreSystem,
    timelimit::TimeLimitSystem,
};

pub fn standard_collision(
    motion_component: &mut Motion2DComponent,
    collision_velocity: Vector2<f32>,
    min_velocity: f32,
) {
    // Case 1: velocities in opposite direction (including entity with 0)
    if (motion_component.velocity.x as i32).signum() != (collision_velocity.x as i32).signum()
        && (collision_velocity.x as i32).signum() != 0
    {
        // if the magnitude of the collision velocity is greater than the minimum velocity
        if collision_velocity.x.abs() > min_velocity {
            motion_component.velocity.x = collision_velocity.x; // set the entity's velocity to the collision velocity
        } else {
            motion_component.velocity.x =
                ((collision_velocity.x as i32).signum() as f32) * min_velocity; // otherwise set the entity's velocity to minimum velocity with the collision velocity's sign
        }
    }
    // Case 2: colliding velocity is 0
    // Case 4: same direction with slower colliding entity
    else if (collision_velocity.x as i32).signum() == 0
        || (motion_component.velocity.x as i32).signum() == (collision_velocity.x as i32).signum()
            && collision_velocity.x.abs() < motion_component.velocity.x.abs()
    {
        motion_component.velocity.x =
            -((motion_component.velocity.x as i32).signum() as f32) * min_velocity;
    }
    // Case 3: same direction with faster colliding velocity
    else if (motion_component.velocity.x as i32).signum()
        == (collision_velocity.x as i32).signum()
        && collision_velocity.x.abs() > motion_component.velocity.x.abs()
    {
        // if the sum of the entity and collision entity's velocity is greater than the minimum velocity
        if (motion_component.velocity.x + collision_velocity.x).abs() > min_velocity {
            motion_component.velocity.x += collision_velocity.x; //add the collision velocity to the entity's velocity
        } else {
            motion_component.velocity.x =
                ((collision_velocity.x as i32).signum() as f32) * min_velocity; // otherwise set the entity's velocity to minimum velocity with the collision velocity's sign
        }
    }

    // Case 1: velocities in opposite direction (including entity with 0)
    if (motion_component.velocity.y as i32).signum() != (collision_velocity.y as i32).signum()
        && (collision_velocity.y as i32).signum() != 0
    {
        // if the magnitude of the collision velocity is greater than the minimum velocity
        if collision_velocity.y.abs() > min_velocity {
            motion_component.velocity.y = collision_velocity.y; // set the entity's velocity to the collision velocity
        } else {
            motion_component.velocity.y =
                ((collision_velocity.y as i32).signum() as f32) * min_velocity; // otherwise set the entity's velocity to minimum velocity with the collision velocity's sign
        }
    }
    // Case 2: colliding velocity is 0
    // Case 4: same direction with slower colliding entity
    else if (collision_velocity.y as i32).signum() == 0
        || (motion_component.velocity.y as i32).signum() == (collision_velocity.y as i32).signum()
            && collision_velocity.y.abs() < motion_component.velocity.y.abs()
    {
        motion_component.velocity.y =
            -((motion_component.velocity.y as i32).signum() as f32) * min_velocity;
    }
    // Case 3: same direction with faster colliding velocity
    else if (motion_component.velocity.y as i32).signum()
        == (collision_velocity.y as i32).signum()
        && collision_velocity.y.abs() > motion_component.velocity.y.abs()
    {
        // if the sum of the entity and collision entity's velocity is greater than the minimum velocity
        if (motion_component.velocity.y + collision_velocity.y).abs() > min_velocity {
            motion_component.velocity.y += collision_velocity.y; //add the collision velocity to the entity's velocity
        } else {
            motion_component.velocity.y =
                ((collision_velocity.y as i32).signum() as f32) * min_velocity; // otherwise set the entity's velocity to minimum velocity with the collision velocity's sign
        }
    }
}

pub fn barrier_collision(
    motion_component: &mut Motion2DComponent,
    barrier_component: &BarrierComponent,
) {
    if barrier_component.deflection_speed.x.abs() > motion_component.velocity.x.abs() {
        motion_component.velocity.x = match barrier_component.push_direction {
            PushDirection::Left => -barrier_component.deflection_speed.x,
            PushDirection::Right => barrier_component.deflection_speed.x,
            _ => motion_component.velocity.x,
        }
    } else {
        motion_component.velocity.x = match barrier_component.push_direction {
            PushDirection::Left => -motion_component.velocity.x.abs(),
            PushDirection::Right => motion_component.velocity.x.abs(),
            _ => motion_component.velocity.x,
        }
    }

    if barrier_component.deflection_speed.y.abs() > motion_component.velocity.y.abs() {
        motion_component.velocity.y = match barrier_component.push_direction {
            PushDirection::Down => -barrier_component.deflection_speed.y,
            PushDirection::Up => barrier_component.deflection_speed.y,
            _ => motion_component.velocity.y,
        }
    } else {
        motion_component.velocity.y = match barrier_component.push_direction {
            PushDirection::Down => -motion_component.velocity.y.abs(),
            PushDirection::Up => motion_component.velocity.y.abs(),
            _ => motion_component.velocity.y,
        }
    }
}

pub fn immovable_collision(
    motion_component: &mut Motion2DComponent,
    collision_velocity: Vector2<f32>,
    min_velocity: f32,
) {
    // Case 1: velocities in opposite direction (including entity with 0)
    if (motion_component.velocity.x as i32).signum() != (collision_velocity.x as i32).signum()
        && (collision_velocity.x as i32).signum() != 0
    {
        if collision_velocity.x.abs() + motion_component.velocity.x.abs() > min_velocity {
            motion_component.velocity.x = ((collision_velocity.x as i32).signum() as f32)
                * (collision_velocity.x.abs() + motion_component.velocity.x.abs())
        } else {
            motion_component.velocity.x =
                ((collision_velocity.x as i32).signum() as f32) * min_velocity;
        }
    }
    // Case 2: colliding velocity is 0
    // Case 4: same direction with slower colliding entity
    else if (collision_velocity.x as i32).signum() == 0
        || (motion_component.velocity.x as i32).signum() == (collision_velocity.x as i32).signum()
            && collision_velocity.x.abs() < motion_component.velocity.x.abs()
    {
        if motion_component.velocity.x.abs() > min_velocity {
            motion_component.velocity.x *= -1.0;
        } else {
            motion_component.velocity.x =
                -((motion_component.velocity.x as i32).signum() as f32) * min_velocity;
        }
    }
    // Case 3: same direction with faster colliding velocity
    else if (motion_component.velocity.x as i32).signum()
        == (collision_velocity.x as i32).signum()
        && collision_velocity.x.abs() > motion_component.velocity.x.abs()
    {
        // if the sum of the entity and collision entity's velocity is greater than the minimum velocity
        if (motion_component.velocity.x + collision_velocity.x).abs() > min_velocity {
            motion_component.velocity.x += collision_velocity.x; //add the collision velocity to the entity's velocity
        } else {
            motion_component.velocity.x =
                ((collision_velocity.x as i32).signum() as f32) * min_velocity; // otherwise set the entity's velocity to minimum velocity with the collision velocity's sign
        }
    }

    // Case 1: velocities in opposite direction (including entity with 0)
    if (motion_component.velocity.y as i32).signum() != (collision_velocity.y as i32).signum()
        && (collision_velocity.y as i32).signum() != 0
    {
        if collision_velocity.y.abs() + motion_component.velocity.y.abs() > min_velocity {
            motion_component.velocity.y = ((collision_velocity.y as i32).signum() as f32)
                * (collision_velocity.y.abs() + motion_component.velocity.y.abs())
        } else {
            motion_component.velocity.y =
                ((collision_velocity.y as i32).signum() as f32) * min_velocity;
        }
    }
    // Case 2: colliding velocity is 0
    // Case 4: same direction with slower colliding entity
    else if (collision_velocity.y as i32).signum() == 0
        || (motion_component.velocity.y as i32).signum() == (collision_velocity.y as i32).signum()
            && collision_velocity.y.abs() < motion_component.velocity.y.abs()
    {
        if motion_component.velocity.y.abs() > min_velocity {
            motion_component.velocity.y *= -1.0;
        } else {
            motion_component.velocity.y =
                -((motion_component.velocity.y as i32).signum() as f32) * min_velocity;
        }
    }
    // Case 3: same direction with faster colliding velocity
    else if (motion_component.velocity.y as i32).signum()
        == (collision_velocity.y as i32).signum()
        && collision_velocity.y.abs() > motion_component.velocity.y.abs()
    {
        // if the sum of the entity and collision entity's velocity is greater than the minimum velocity
        if (motion_component.velocity.y + collision_velocity.y).abs() > min_velocity {
            motion_component.velocity.y += collision_velocity.y; //add the collision velocity to the entity's velocity
        } else {
            motion_component.velocity.y =
                ((collision_velocity.y as i32).signum() as f32) * min_velocity; // otherwise set the entity's velocity to minimum velocity with the collision velocity's sign
        }
    }
}
