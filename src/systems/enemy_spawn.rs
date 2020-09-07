use crate::{
    components::{BossType, EnemySpawnerTag, GameMaster, PhaseType, Spawner},
    entities::{spawn_enemy, spawn_repeater},
    resources::{EnemyPool, SpriteResource},
};
use amethyst::{
    core::{math::Vector3, timing::Time, Transform},
    ecs::{Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteStorage},
};

pub struct SpawnerSystem;

impl<'s> System<'s> for SpawnerSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Spawner>,
        ReadStorage<'s, EnemySpawnerTag>,
        Read<'s, Time>,
        ReadExpect<'s, SpriteResource>,
        WriteStorage<'s, GameMaster>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, EnemyPool>,
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
            mut gamemasters,
            lazy_update,
            enemy_pool,
        ): Self::SystemData,
    ) {
        for gamemaster in (&mut gamemasters).join() {
            if gamemaster.phase_idx < gamemaster.last_phase {
                match gamemaster.phase_map[gamemaster.phase_idx].phase_type {
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
                                    enemy_resource.enemy_animations_sprite_sheet.clone(),
                                    enemy_pool[name].clone(),
                                    spawn_position,
                                    &lazy_update,
                                );
                            }
                        }
                    }

                    PhaseType::Boss => {
                        match gamemaster.phase_map[gamemaster.phase_idx].boss_type {
                            BossType::Repeater => {
                                // spawn repeater boss
                                if !gamemaster.phase_map[gamemaster.phase_idx].boss_spawned {
                                    spawn_repeater(
                                        &entities,
                                        enemy_resource.repeater_sprite_sheet.clone(),
                                        &enemy_pool,
                                        &lazy_update,
                                    );

                                    gamemaster.phase_map[gamemaster.phase_idx].boss_spawned = true;
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
}
