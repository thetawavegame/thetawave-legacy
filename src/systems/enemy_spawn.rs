use amethyst::{
    core::{
        Transform,
        timing::Time,
        math::Vector3,
    },
    ecs::{Join, Read, ReadStorage, System, WriteStorage, Entities, LazyUpdate, ReadExpect},
};

use crate::{
    entities::{spawn_enemy, spawn_boss},
    components::{Spawner, Enemy, GameMaster, PhaseType},
    resources::SpriteResource,
    space_shooter::{ARENA_MIN_X, ARENA_WIDTH, ARENA_HEIGHT},
};


pub struct SpawnerSystem;

impl<'s> System<'s> for SpawnerSystem {

    type SystemData  = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Spawner<Enemy>>,
        Read<'s, Time>,
        ReadExpect<'s, SpriteResource>,
        WriteStorage<'s, GameMaster>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut transforms, mut spawners, time, enemy_resource, mut gamemasters, lazy_update): Self::SystemData) {
        for gamemaster in (&mut gamemasters).join() {

            if gamemaster.phase_idx < gamemaster.last_phase {
            
                match gamemaster.phase_map[gamemaster.phase_idx].phase_type {
                    PhaseType::Invasion => {

                        for (spawner, transform) in (&mut spawners, &mut transforms).join() {
                            if let Some(new_x) = spawner.can_spawn(time.delta_seconds()) {

                                let spawn_position = Vector3::new(
                                    new_x, transform.translation()[1], transform.translation()[2]
                                );
                                spawn_enemy(&entities, &enemy_resource, &mut spawner.pool, spawn_position, &lazy_update);
                            }
                        }

                    }

                    PhaseType::Boss => {
                        if !gamemaster.phase_map[gamemaster.phase_idx].boss_spawned {
                            let spawn_position = Vector3::new(
                                    ARENA_MIN_X + (ARENA_WIDTH/2.0), ARENA_HEIGHT/1.2, 0.0,
                                );
                            spawn_boss(&entities, &enemy_resource, spawn_position, &lazy_update);
                            println!("repeater spawned!");
                            gamemaster.phase_map[gamemaster.phase_idx].boss_spawned = true;
                        }
                    }

                    PhaseType::Rest => {}
                }
            }
        }

    }
}