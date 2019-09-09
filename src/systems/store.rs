use amethyst::{
    core::{
        Transform,
        timing::Time,
        math::Vector3,
    },
    ecs::{Join, Read, System, WriteStorage, ReadStorage, Entities, LazyUpdate, ReadExpect},
    ecs::prelude::World,
    input::{InputHandler, StringBindings},
};

use crate::{
    entities::{spawn_item},
    components::{Spawner, GameMaster, PhaseType, Store, Spaceship},
    resources::{SpriteResource, ItemPool},
};



pub struct StoreSystem;

impl<'s> System<'s> for StoreSystem {

    type SystemData  = (
        Entities<'s>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, ItemPool>,
        WriteStorage<'s, Store>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, Spaceship>,
    );

    fn run(&mut self, (entities, sprite_resource, lazy_update, item_pool, mut stores, time, input, mut spaceships): Self::SystemData) {

        let buy_1_action = input.action_is_down("buy_1").unwrap();
        let buy_2_action = input.action_is_down("buy_2").unwrap();
        let buy_3_action = input.action_is_down("buy_3").unwrap();

        for (store) in (&mut stores).join() {
            store.restock(time.delta_seconds(), item_pool.clone(), &entities, &sprite_resource, &lazy_update);

            for spaceship in (&mut spaceships).join() {
                if buy_1_action {
                    println!("buy store item 1");
                    store.purchase_item(0, &entities, spaceship, &sprite_resource, &lazy_update);
                }else if buy_2_action {
                    println!("buy store item 2");
                    store.purchase_item(1, &entities, spaceship, &sprite_resource, &lazy_update);
                }else if buy_3_action {
                    println!("buy store item 3");
                    store.purchase_item(2, &entities, spaceship, &sprite_resource, &lazy_update);
                }
            }

        }

    }
}