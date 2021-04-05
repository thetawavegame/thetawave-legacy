use crate::{
    components::AutoSpawnerComponent,
    entities::spawn_repeater,
    resources::{
        BossType, ConsumablesResource, EffectsResource, EnemiesResource, ItemsResource,
        PhaseManagerResource, PhaseType, SpawnerResource, SpriteSheetsResource,
    },
};
use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::{
        Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, Write, WriteExpect,
        WriteStorage,
    },
};

pub struct SpawnerSystem;

impl<'s> System<'s> for SpawnerSystem {
    type SystemData = (
        Entities<'s>,
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
        match &phase_manager.phase_map[phase_manager.phase_idx].phase_type {
            PhaseType::InvasionRandom(random_pool_type) => spawner_resource
                .spawn_random_spawnable_when_ready(
                    &random_pool_type,
                    time.delta_seconds(),
                    &consumables_resource,
                    &enemies_resource,
                    &items_resource,
                    &effects_resource,
                    &spritesheets_resource,
                    &entities,
                    &lazy_update,
                ),

            PhaseType::InvasionFormation(formation_pool_type) => spawner_resource
                .spawn_random_formation_when_ready(
                    &&formation_pool_type,
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

pub struct AutoSpawnerSystem;

impl<'s> System<'s> for AutoSpawnerSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, AutoSpawnerComponent>,
        Read<'s, Time>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, EnemiesResource>,
        ReadExpect<'s, ConsumablesResource>,
        ReadExpect<'s, ItemsResource>,
        ReadExpect<'s, EffectsResource>,
        ReadExpect<'s, SpriteSheetsResource>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (
            transforms,
            mut auto_child_entity_spawners,
            time,
            lazy_update,
            enemies_resource,
            consumables_resource,
            items_resource,
            effects_resource,
            sprite_sheets_resource,
            entities,
        ): Self::SystemData,
    ) {
        for (transform, auto_child_entity_spawner) in
            (&transforms, &mut auto_child_entity_spawners).join()
        {
            auto_child_entity_spawner.spawn_when_ready(
                time.delta_seconds(),
                transform.clone(),
                &sprite_sheets_resource,
                &enemies_resource,
                &consumables_resource,
                &items_resource,
                &effects_resource,
                &entities,
                &lazy_update,
            );
        }
    }
}
