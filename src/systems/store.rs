use amethyst::{
    core::{
        Transform,
        timing::Time,
        math::Vector3,
    },
    ecs::{Join, Read, System, WriteStorage, ReadStorage, Entities, LazyUpdate, ReadExpect},
    ecs::prelude::World,
    input::{InputHandler, StringBindings},
    audio::{output::Output, Source},
    assets::AssetStorage,
};

use crate::{
    entities::{spawn_item},
    components::{Spawner, GameMaster, PhaseType, Store, Spaceship},
    resources::{SpriteResource, ItemPool},
    audio::{play_sfx, Sounds},
};

use std::ops::Deref;



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
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(&mut self, (entities, sprite_resource, lazy_update, item_pool, mut stores, time, input, mut spaceships, storage, sounds, audio_output): Self::SystemData) {

        let buy_0_action = input.action_is_down("buy_0").unwrap();
        let buy_1_action = input.action_is_down("buy_1").unwrap();
        let buy_2_action = input.action_is_down("buy_2").unwrap();

        for (store) in (&mut stores).join() {
            store.restock(time.delta_seconds(), item_pool.clone(), &entities, &sprite_resource, &lazy_update);

            for spaceship in (&mut spaceships).join() {
                if buy_0_action {
                    if store.purchase_item(0, &entities, spaceship, &sprite_resource, &lazy_update) {
                        play_sfx(&sounds.cash_register_bell, &storage, audio_output.as_ref().map(|o| o.deref()));
                    }
                }else if buy_1_action {
                    if store.purchase_item(1, &entities, spaceship, &sprite_resource, &lazy_update) {
                        play_sfx(&sounds.cash_register_bell, &storage, audio_output.as_ref().map(|o| o.deref()));
                    }
                }else if buy_2_action {
                    if store.purchase_item(2, &entities, spaceship, &sprite_resource, &lazy_update) {
                        play_sfx(&sounds.cash_register_bell, &storage, audio_output.as_ref().map(|o| o.deref()));
                    }
                }
            }

        }

    }
}