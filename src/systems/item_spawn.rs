use amethyst::{
    core::{
        Transform,
        timing::Time,
        math::Vector3,
    },
    ecs::{Join, Read, System, WriteStorage, ReadStorage, Entities, LazyUpdate, ReadExpect},
};
use crate::{
    entities::{spawn_item},
    components::{Spawner, ItemSpawnerTag, GameMaster, PhaseType},
    resources::{SpriteResource, ItemPool},
};

pub struct ItemSpawnSystem;

impl<'s> System<'s> for ItemSpawnSystem {

    type SystemData  = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Spawner>,
        ReadStorage<'s, ItemSpawnerTag>,
        Read<'s, Time>,
        ReadExpect<'s, SpriteResource>,
        ReadStorage<'s, GameMaster>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, ItemPool>
    );

    fn run(&mut self, (entities, mut transforms, mut spawners, spawner_tag, time, sprite_resource, gamemasters, lazy_update, item_pool): Self::SystemData) {

        for gamemaster in (&gamemasters).join() {
            if gamemaster.phase_idx < gamemaster.last_phase {

                match gamemaster.phase_map[gamemaster.phase_idx].phase_type {
                    PhaseType::Invasion => {

                        for (spawner, transform, _) in (&mut spawners, &mut transforms, &spawner_tag).join() {
                            if let Some((new_x, name)) = spawner.spawn_with_position(time.delta_seconds()) {
                                let spawn_position = Vector3::new(
                                    new_x, transform.translation()[1], transform.translation()[2]
                                );
                                spawn_item(&entities, &sprite_resource, item_pool[name].clone(), spawn_position, &lazy_update);
                                let name = name.clone();// clone so i could borrow spawner
                                // only spawn one time for each item type
                                spawner.disable_item(&name);
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