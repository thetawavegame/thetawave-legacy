use amethyst::{
    ecs::prelude::{World, Builder},
    core::transform::Transform,
};

use crate::components::{Spawner, ItemSpawnerTag};
use crate::resources::ItemPool;

use crate::space_shooter::{ARENA_MAX_Y, ARENA_MIN_X, ARENA_WIDTH};

const SPAWN_INTERVAL: f32 = 10.0;
const SPAWN_COUNT: u32 = 10;
const SPAWNER_Y_OFFSET: f32 = 20.0;

pub fn initialise_item_spawner(world: &mut World) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_MIN_X + (ARENA_WIDTH / 2.0), ARENA_MAX_Y + SPAWNER_Y_OFFSET, 0.0);

    // create spawn list, one item for each item type
    let spawn_list = world.read_resource::<ItemPool>()
        .iter()
        .map(|(key,_)| (key.clone(), 1.0)) // all with same probability
        .collect();

    world
        .create_entity()
        .with(Spawner::new(spawn_list, SPAWN_INTERVAL, SPAWN_COUNT))
        .with(ItemSpawnerTag)
        .with(local_transform)
        .build();
}