use amethyst::{
    core::{
        Transform,
        timing::Time,
        math::Vector3,
    },
    ecs::{Join, Read, ReadStorage, System, WriteStorage, Entities, LazyUpdate, ReadExpect},
};

use crate::{
    entities::{spawn_enemy},
    components::{Spawner, Enemy, GameMaster, PhaseType},
    resources::SpriteResource,
};


pub struct SpawnerSystem;

impl<'s> System<'s> for SpawnerSystem {

    type SystemData  = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Spawner<Enemy>>,
        Read<'s, Time>,
        ReadExpect<'s, SpriteResource>,
        ReadStorage<'s, GameMaster>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut transforms, mut spawners, time, enemy_resource, gamemasters, lazy_update): Self::SystemData) {
        for gamemaster in (gamemasters).join() {

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

                    PhaseType::Boss => {}

                    PhaseType::Rest => {}
                }
            }
        }

    }
}