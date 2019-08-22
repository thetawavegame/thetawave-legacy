use amethyst::{
    core::{
        Transform,
        timing::Time,
        math::Vector3,
    },
    ecs::{Join, Read, System, WriteStorage, Entities, LazyUpdate, ReadExpect},
};

use crate::{
    entities::{spawn_enemy},
    components::{Spawner, Enemy},
    resources::SpriteResource,
};


pub struct SpawnerSystem;

impl<'s> System<'s> for SpawnerSystem {

    type SystemData  = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Spawner<Enemy>>,
        Read<'s, Time>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut transforms, mut spawners, time, enemy_resource, lazy_update): Self::SystemData) {

        for (spawner, transform) in (&mut spawners, &mut transforms).join() {
            if let Some(new_x) = spawner.can_spawn(time.delta_seconds()) {

                let spawn_position = Vector3::new(
                    new_x, transform.translation()[1], transform.translation()[2]
                );
                spawn_enemy(&entities, &enemy_resource, &mut spawner.pool, spawn_position, &lazy_update);
            }
        }
    }
}