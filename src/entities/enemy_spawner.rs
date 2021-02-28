use crate::{
    components::{EnemySpawnerTag, SpawnerComponent},
    constants::{
        ARENA_MAX_Y, ARENA_MIN_X, ARENA_WIDTH, ENEMY_DRONE_RATIO, ENEMY_HAULER_RATIO,
        ENEMY_MISSILE_LAUNCHER_RATIO, ENEMY_MISSILE_RATIO, ENEMY_PAWN_RATIO, ENEMY_SPAWN_INTERVAL,
        ENEMY_STRAFER_RATIO, SPAWNER_Y_OFFSET,
    },
    entities::EntityType,
};
use amethyst::{
    core::transform::Transform,
    ecs::{World, WorldExt},
    prelude::Builder,
};

pub fn initialize_enemy_spawner(world: &mut World) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(
        ARENA_MIN_X + (ARENA_WIDTH / 2.0),
        ARENA_MAX_Y + SPAWNER_Y_OFFSET,
        0.0,
    );
    world
        .create_entity()
        .with(SpawnerComponent::new(
            vec![
                (Some(EntityType::Pawn), ENEMY_PAWN_RATIO),
                (Some(EntityType::Drone), ENEMY_DRONE_RATIO),
                (Some(EntityType::Hauler), ENEMY_HAULER_RATIO),
                (Some(EntityType::Strafer), ENEMY_STRAFER_RATIO),
                (
                    Some(EntityType::MissileLauncher),
                    ENEMY_MISSILE_LAUNCHER_RATIO,
                ),
            ],
            ENEMY_SPAWN_INTERVAL,
        ))
        .with(EnemySpawnerTag)
        .with(local_transform)
        .build();
}
