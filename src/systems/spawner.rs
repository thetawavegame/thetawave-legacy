use crate::{
    components::{EnemySpawnerTag, SpawnerComponent},
    entities::{spawn_enemy, spawn_repeater, SpawnableType},
    resources::{
        BossType, ConsumablesResource, EffectsResource, EnemiesResource, ItemsResource,
        PhaseManagerResource, PhaseType, SpawnerResource, SpriteSheetsResource,
    },
};
use amethyst::{
    core::{timing::Time, Transform},
    ecs::{
        Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, Write, WriteExpect,
        WriteStorage,
    },
};

pub struct SpawnerSystem;

impl<'s> System<'s> for SpawnerSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpawnerComponent>,
        ReadStorage<'s, EnemySpawnerTag>,
        Read<'s, Time>,
        WriteExpect<'s, SpawnerResource>,
        ReadExpect<'s, SpriteSheetsResource>,
        Write<'s, PhaseManagerResource>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, ConsumablesResource>,
        ReadExpect<'s, ItemsResource>,
        ReadExpect<'s, EffectsResource>,
        ReadExpect<'s, EnemiesResource>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut transforms,
            mut spawners,
            spawner_tag,
            time,
            mut spawner_resource,
            spritesheets_resource,
            mut phase_manager,
            lazy_update,
            consumables_resource,
            items_resource,
            effects_resource,
            enemies_resource,
        ): Self::SystemData,
    ) {
        if phase_manager.phase_idx < phase_manager.last_phase {
            match phase_manager.phase_map[phase_manager.phase_idx].phase_type {
                PhaseType::RandomInvasion => spawner_resource.spawn_random_spawnable_when_ready(
                    time.delta_seconds(),
                    &consumables_resource,
                    &enemies_resource,
                    &items_resource,
                    &effects_resource,
                    &spritesheets_resource,
                    &entities,
                    &lazy_update,
                ),

                PhaseType::FormationInvasion => spawner_resource.spawn_random_formation_when_ready(
                    time.delta_seconds(),
                    &consumables_resource,
                    &enemies_resource,
                    &items_resource,
                    &effects_resource,
                    &spritesheets_resource,
                    &entities,
                    &lazy_update,
                ),

                PhaseType::Boss => {
                    match phase_manager.phase_map[phase_manager.phase_idx].boss_type {
                        BossType::Repeater => {
                            // spawn repeater boss
                            if !phase_manager.phase_map[phase_manager.phase_idx].boss_spawned {
                                spawn_repeater(
                                    &spritesheets_resource,
                                    &enemies_resource,
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
