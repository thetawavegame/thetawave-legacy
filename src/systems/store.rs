use amethyst::{
    core::{
        Transform,
        timing::Time,
        math::Vector3,
    },
    ecs::{Join, Read, System, WriteStorage, ReadStorage, Entities, LazyUpdate, ReadExpect},
    ecs::prelude::World,
};

use crate::{
    entities::{spawn_item},
    components::{Spawner, GameMaster, PhaseType, Store},
    resources::{SpriteResource, ItemPool},
};


pub struct StoreSystem;

impl<'s> System<'s> for StoreSystem {

    type SystemData  = (
        ReadExpect<'s, ItemPool>,
        WriteStorage<'s, Store>,
        Read<'s, Time>,
    );

    fn run(&mut self, (item_pool, mut stores, time): Self::SystemData) {

        for store in (&mut stores).join() {
            store.restock(time.delta_seconds(), item_pool.clone());
        }

    }
}