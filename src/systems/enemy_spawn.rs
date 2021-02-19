use crate::{
    components::{EnemySpawnerTag, SpawnerComponent},
    entities::{spawn_enemy, spawn_repeater},
    resources::{
        BossType, EnemiesResource, PhaseManagerResource, PhaseType, SpriteSheetsResource,
        ThrustersResource,
    },
};
use amethyst::{
    core::{math::Vector3, timing::Time, Transform},
    ecs::{Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, Write, WriteStorage},
};

pub struct SpawnerSystem;

impl<'s> System<'s> for SpawnerSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpawnerComponent>,
        ReadStorage<'s, EnemySpawnerTag>,
        Read<'s, Time>,
        ReadExpect<'s, SpriteSheetsResource>,
        Write<'s, PhaseManagerResource>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, EnemiesResource>,
        ReadExpect<'s, ThrustersResource>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut transforms,
            mut spawners,
            spawner_tag,
            time,
            enemy_resource,
            mut phase_manager,
            lazy_update,
            enemy_pool,
            thruster_pool,
        ): Self::SystemData,
    ) {
        if phase_manager.phase_idx < phase_manager.last_phase {
            match phase_manager.phase_map[phase_manager.phase_idx].phase_type {
                PhaseType::Invasion => {
                    for (spawner, transform, _) in
                        (&mut spawners, &mut transforms, &spawner_tag).join()
                    {
                        if let Some((new_x, name)) =
                            spawner.spawn_with_position(time.delta_seconds())
                        {
                            let spawn_position = Vector3::new(
                                new_x,
                                transform.translation()[1],
                                transform.translation()[2],
                            );

                            spawn_enemy(
                                &entities,
                                enemy_resource.spritesheets["enemies"].clone(),
                                Some(enemy_resource.spritesheets["thrusters"].clone()),
                                enemy_pool[name].clone(),
                                Some(thruster_pool[name].clone()),
                                spawn_position,
                                &lazy_update,
                            );
                        }
                    }
                }

                PhaseType::Boss => {
                    match phase_manager.phase_map[phase_manager.phase_idx].boss_type {
                        BossType::Repeater => {
                            // spawn repeater boss
                            if !phase_manager.phase_map[phase_manager.phase_idx].boss_spawned {
                                spawn_repeater(
                                    &entities,
                                    enemy_resource.spritesheets["repeater"].clone(),
                                    &enemy_pool,
                                    &lazy_update,
                                );
                                let phase_idx = phase_manager.phase_idx;
                                phase_manager.phase_map[phase_idx].boss_spawned = true;
                            }
                        }

                        BossType::None => {}
                    }
                }

                PhaseType::Rest => {}
            }
        }
    }
}
