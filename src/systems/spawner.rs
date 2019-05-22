use amethyst::{
    core::{
        Transform,
        timing::Time,
        nalgebra::Vector3,
    },
    ecs::{Join, Read, System, WriteStorage, Entities, LazyUpdate, ReadExpect},
};

use crate::{
    entities::{spawn_enemy, ENEMY_WIDTH},
    components::Spawner,
    resources::EnemyResource,
    space_shooter::GAME_WIDTH,
};

use rand::{thread_rng, Rng};


pub struct SpawnerSystem;

impl<'s> System<'s> for SpawnerSystem {

    type SystemData  = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Spawner>,
        Read<'s, Time>,
        ReadExpect<'s, EnemyResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut transforms, mut spawners, time, enemy_resource, lazy_update): Self::SystemData) {

        for (spawner, transform) in (&mut spawners, &mut transforms).join() {

            if spawner.spawn_timer > 0.0 {
                spawner.spawn_timer -= time.delta_seconds();
            } else {


                let max_width = GAME_WIDTH - ENEMY_WIDTH;
                let min_width = ENEMY_WIDTH;
                let new_x = ENEMY_WIDTH + thread_rng().gen::<f32>()* (max_width - min_width);


                let spawn_position = Vector3::new(
                    new_x, transform.translation()[1], transform.translation()[2],
                );

                spawn_enemy(&entities, &enemy_resource, spawn_position, &lazy_update);

                spawner.spawn_timer = spawner.spawn_interval;
            }
        }
    }
}