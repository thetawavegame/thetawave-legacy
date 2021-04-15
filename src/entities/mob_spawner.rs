use crate::{
    components::{MobSpawnerTag, SpawnerComponent},
    constants::{
        ALLY_HAULER_RATIO, ARENA_MAX_Y, ARENA_MIN_X, ARENA_WIDTH, ENEMY_DRONE_RATIO,
        ENEMY_MISSILE_LAUNCHER_RATIO, ENEMY_PAWN_RATIO, ENEMY_SPAWN_INTERVAL, ENEMY_STRAFER_RATIO,
        SPAWNER_Y_OFFSET,
    },
    entities::{AllyType, EnemyType, MobType, SpawnableType},
};
use amethyst::{
    core::transform::Transform,
    ecs::{World, WorldExt},
    prelude::Builder,
};

pub fn initialize_mob_spawner(world: &mut World) {
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
                (
                    Some(SpawnableType::Mob(MobType::Enemy(EnemyType::Pawn))),
                    ENEMY_PAWN_RATIO,
                ),
                (
                    Some(SpawnableType::Mob(MobType::Enemy(EnemyType::Drone))),
                    ENEMY_DRONE_RATIO,
                ),
                (
                    Some(SpawnableType::Mob(MobType::Ally(AllyType::Hauler))),
                    ALLY_HAULER_RATIO,
                ),
                (
                    Some(SpawnableType::Mob(MobType::Enemy(EnemyType::StraferRight))),
                    ENEMY_STRAFER_RATIO,
                ),
                (
                    Some(SpawnableType::Mob(MobType::Enemy(EnemyType::StraferLeft))),
                    ENEMY_STRAFER_RATIO,
                ),
                (
                    Some(SpawnableType::Mob(MobType::Enemy(
                        EnemyType::MissileLauncher,
                    ))),
                    ENEMY_MISSILE_LAUNCHER_RATIO,
                ),
            ],
            ENEMY_SPAWN_INTERVAL,
        ))
        .with(MobSpawnerTag)
        .with(local_transform)
        .build();
}
