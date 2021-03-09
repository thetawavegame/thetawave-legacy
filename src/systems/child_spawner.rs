use crate::{
    components::AutoChildEntitySpawnerComponent,
    resources::{
        ConsumablesResource, EffectsResource, EnemiesResource, ItemsResource, SpriteSheetsResource,
    },
};
use amethyst::{
    core::{math::Vector3, timing::Time, Transform},
    ecs::{Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteStorage},
};

pub struct AutoChildEntitySpawnerSystem;

impl<'s> System<'s> for AutoChildEntitySpawnerSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, AutoChildEntitySpawnerComponent>,
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
