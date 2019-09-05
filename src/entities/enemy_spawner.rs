use amethyst::{core::transform::Transform, ecs::prelude::World, prelude::Builder};

use crate::components::{EnemySpawnerTag, Spawner};

use crate::space_shooter::{ARENA_MAX_Y, ARENA_MIN_X, ARENA_WIDTH};

const SPAWN_INTERVAL: f32 = 1.5;
const SPAWN_COUNTER: u32 = 100;

const ENEMY_PAWN_RATIO: f32 = 50.0;
const ENEMY_DRONE_RATIO: f32 = 100.0;
const ENEMY_HAULER_RATIO: f32 = 5.0;

const SPAWNER_Y_OFFSET: f32 = 20.0;

pub fn initialise_enemy_spawner(world: &mut World) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(
        ARENA_MIN_X + (ARENA_WIDTH / 2.0),
        ARENA_MAX_Y + SPAWNER_Y_OFFSET,
        0.0,
    );
    world
        .create_entity()
        .with(Spawner::new(
            vec![
                ("pawn".to_string(), ENEMY_PAWN_RATIO),
                ("drone".to_string(), ENEMY_DRONE_RATIO),
                ("hauler".to_string(), ENEMY_HAULER_RATIO),
            ],
            SPAWN_INTERVAL,
            SPAWN_COUNTER,
        ))
        .with(EnemySpawnerTag)
        .with(local_transform)
        .build();
}
