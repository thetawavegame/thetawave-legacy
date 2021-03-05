use crate::{
    components::{AutoChildEnemySpawnerComponent, EnemySpawnerTag, SpawnerComponent},
    entities::{spawn_enemy, spawn_repeater, EntityType},
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
            sprite_sheets_resource,
            mut phase_manager,
            lazy_update,
            enemies_resource,
            thrusters_resource,
        ): Self::SystemData,
    ) {
        if phase_manager.phase_idx < phase_manager.last_phase {
            match phase_manager.phase_map[phase_manager.phase_idx].phase_type {
                PhaseType::Invasion => {
                    for (spawner, transform, _) in
                        (&mut spawners, &mut transforms, &spawner_tag).join()
                    {
                        if let Some((new_x, Some(enemy_type))) =
                            spawner.spawn_with_position(time.delta_seconds())
                        {
                            let spawn_position = Vector3::new(
                                new_x,
                                transform.translation()[1],
                                transform.translation()[2],
                            );

                            if let EntityType::Enemy(enemy_type) = enemy_type {
                                spawn_enemy(
                                    enemy_type,
                                    &sprite_sheets_resource,
                                    &enemies_resource,
                                    &thrusters_resource,
                                    spawn_position,
                                    &entities,
                                    &lazy_update,
                                );
                            }
                        }
                    }
                }

                PhaseType::Boss => {
                    match phase_manager.phase_map[phase_manager.phase_idx].boss_type {
                        BossType::Repeater => {
                            // spawn repeater boss
                            if !phase_manager.phase_map[phase_manager.phase_idx].boss_spawned {
                                spawn_repeater(
                                    &sprite_sheets_resource,
                                    &enemies_resource,
                                    &thrusters_resource,
                                    &entities,
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

pub struct AutoChildEnemySpawnerSystem;

impl<'s> System<'s> for AutoChildEnemySpawnerSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, AutoChildEnemySpawnerComponent>,
        Read<'s, Time>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, EnemiesResource>,
        ReadExpect<'s, ThrustersResource>,
        ReadExpect<'s, SpriteSheetsResource>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (
            transforms,
            mut auto_child_enemy_spawners,
            time,
            lazy_update,
            enemies_resource,
            thrusters_resource,
            sprite_sheets_resource,
            entities,
        ): Self::SystemData,
    ) {
        for (transform, auto_child_enemy_spawner) in
            (&transforms, &mut auto_child_enemy_spawners).join()
        {
            let spawn_position = Vector3::new(
                transform.translation().x,
                transform.translation().y,
                transform.translation().z,
            );
            auto_child_enemy_spawner.spawn_when_ready(
                time.delta_seconds(),
                spawn_position,
                &sprite_sheets_resource,
                &enemies_resource,
                &thrusters_resource,
                &entities,
                &lazy_update,
            );
        }
    }
}
