use amethyst::{
    core::{
        Transform,
        timing::Time,
        math::Vector3,
    },
    ecs::{Join, Read, System, WriteStorage, Entities, LazyUpdate, ReadExpect},
};

use crate::{
    entities::{spawn_item},
    components::{Spawner, Item},
    resources::SpriteResource,
};


pub struct ItemSpawnSystem;

impl<'s> System<'s> for ItemSpawnSystem {

    type SystemData  = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Spawner<Item>>,
        Read<'s, Time>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut transforms, mut spawners, time, sprite_resource, lazy_update): Self::SystemData) {

        for (spawner, transform) in (&mut spawners, &mut transforms).join() {
            if let Some(new_x) = spawner.can_spawn(time.delta_seconds()) {

                let spawn_position = Vector3::new(
                    new_x, transform.translation()[1], transform.translation()[2]
                );
                spawn_item(&entities, &sprite_resource, &mut spawner.pool, spawn_position, &lazy_update);
            }
        }
    }
}