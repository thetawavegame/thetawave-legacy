use amethyst::{
    prelude::Builder,
    ecs::prelude::World,
    core::transform::Transform,
};

use crate::{
    components::Spawner,
    space_shooter::{GAME_HEIGHT, GAME_WIDTH},
};


const SPAWN_INTERVAL: f32 = 0.8;


pub fn initialise_spawner(world: &mut World) {

    //create transform
    let mut local_transform = Transform::default();
    local_transform.set_xyz(GAME_WIDTH / 2.0, GAME_HEIGHT, 0.0);

    world
        .create_entity()
        .with(Spawner {
            spawn_timer: 0.0,
            spawn_interval: SPAWN_INTERVAL,
        })
        .with(local_transform)
        .build();
}