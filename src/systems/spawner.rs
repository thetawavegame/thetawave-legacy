use crate::{
    components::{MobSpawnerTag, SpawnerComponent},
    entities::{spawn_mob, spawn_repeater, SpawnableType},
    resources::{
        BossType, ConsumablesResource, EffectsResource, FormationsResource, ItemsResource,
        MobsResource, PhaseManagerResource, PhaseType, SpriteSheetsResource,
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
        ReadStorage<'s, MobSpawnerTag>,
        Read<'s, Time>,
        WriteExpect<'s, FormationsResource>,
        ReadExpect<'s, SpriteSheetsResource>,
        Write<'s, PhaseManagerResource>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, ConsumablesResource>,
        ReadExpect<'s, ItemsResource>,
        ReadExpect<'s, EffectsResource>,
        ReadExpect<'s, MobsResource>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut transforms,
            mut spawners,
            spawner_tag,
            time,
            mut formations_resource,
            spritesheets_resource,
            mut phase_manager,
            lazy_update,
            consumables_resource,
            items_resource,
            effects_resource,
            mobs_resource,
        ): Self::SystemData,
    ) {
        if phase_manager.phase_idx < phase_manager.last_phase {
            match phase_manager.phase_map[phase_manager.phase_idx].phase_type {
                PhaseType::RandomInvasion => {
                    for (spawner, transform, _) in
                        (&mut spawners, &mut transforms, &spawner_tag).join()
                    {
                        if let Some((new_x, Some(spawnable_type))) =
                            spawner.spawn_with_position(time.delta_seconds())
                        {
                            let mut spawn_transform = Transform::default();
                            spawn_transform.set_translation_xyz(
                                new_x,
                                transform.translation()[1],
                                transform.translation()[2],
                            );

                            if let SpawnableType::Mob(mob_type) = spawnable_type {
                                spawn_mob(
                                    mob_type,
                                    spawn_transform,
                                    &mobs_resource,
                                    &spritesheets_resource,
                                    &entities,
                                    &lazy_update,
                                );
                            }
                        }
                    }
                }

                PhaseType::FormationInvasion => formations_resource
                    .spawn_random_formation_when_ready(
                        time.delta_seconds(),
                        &consumables_resource,
                        &mobs_resource,
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
                                    &mobs_resource,
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
