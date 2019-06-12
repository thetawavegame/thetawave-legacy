use amethyst::{
    core::{
        Transform,
        timing::Time,
        nalgebra::Vector3,
    },
    ecs::{Join, Read, System, WriteStorage, Entities, LazyUpdate, ReadExpect},
};

use rand::{thread_rng, Rng};

use crate::{
    entities::{spawn_item},
    components::ItemSpawner,
    resources::SpriteResource,
};
use crate::space_shooter::{ARENA_MAX_X, ARENA_MIN_X, ARENA_SPAWN_OFFSET};


pub struct ItemSpawnSystem;

impl<'s> System<'s> for ItemSpawnSystem {

    type SystemData  = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, ItemSpawner>,
        Read<'s, Time>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut transforms, mut spawners, time, sprite_resource, lazy_update): Self::SystemData) {

        for (spawner, transform) in (&mut spawners, &mut transforms).join() {
            if spawner.spawn_timer > 0.0 {
                spawner.spawn_timer -= time.delta_seconds();
            } else {

                let max_width = ARENA_MAX_X - ARENA_SPAWN_OFFSET;
                let min_width = ARENA_MIN_X + ARENA_SPAWN_OFFSET;
                let new_x = ARENA_MIN_X + ARENA_SPAWN_OFFSET + thread_rng().gen::<f32>()* (max_width - min_width);


                let spawn_position = Vector3::new(
                    new_x, transform.translation()[1], transform.translation()[2],
                );

                if spawner.item_pool.available_items.len() > 0 {
                    spawn_item(&entities, &sprite_resource, &mut spawner.item_pool, spawn_position, &lazy_update);
                }

                spawner.spawn_timer = spawner.spawn_interval;
            }
        }
    }
}