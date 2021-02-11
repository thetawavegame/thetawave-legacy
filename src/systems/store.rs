use crate::{
    audio::Sounds,
    components::{PlayerComponent, StoreComponent},
    events::PlayAudioEvent,
    resources::{ItemsResource, SpriteSheetsResource},
};
use amethyst::{
    core::{timing::Time, Transform},
    ecs::{Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    shrev::EventChannel,
};

pub struct StoreSystem;

impl<'s> System<'s> for StoreSystem {
    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, SpriteSheetsResource>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, ItemsResource>,
        WriteStorage<'s, StoreComponent>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, PlayerComponent>,
        ReadStorage<'s, Transform>,
        Write<'s, EventChannel<PlayAudioEvent>>,
        ReadExpect<'s, Sounds>,
    );

    fn run(
        &mut self,
        (
            entities,
            sprite_resource,
            lazy_update,
            item_pool,
            mut stores,
            time,
            input,
            mut players,
            transforms,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        let buy_0_action = input.action_is_down("buy_0").unwrap();
        let buy_1_action = input.action_is_down("buy_1").unwrap();
        let buy_2_action = input.action_is_down("buy_2").unwrap();

        for store in (&mut stores).join() {
            store.restock(
                time.delta_seconds(),
                item_pool.clone(),
                &entities,
                &sprite_resource,
                &lazy_update,
            );

            // TODO: streamline purchase_item function with constant component in item data file
            for (character, transform) in (&mut players, &transforms).join() {
                if (buy_0_action
                    && store.purchase_item(
                        0,
                        &entities,
                        character,
                        transform,
                        &item_pool,
                        &sprite_resource,
                        &lazy_update,
                    ))
                    || (buy_1_action
                        && store.purchase_item(
                            1,
                            &entities,
                            character,
                            transform,
                            &item_pool,
                            &sprite_resource,
                            &lazy_update,
                        ))
                    || (buy_2_action
                        && store.purchase_item(
                            2,
                            &entities,
                            character,
                            transform,
                            &item_pool,
                            &sprite_resource,
                            &lazy_update,
                        ))
                {
                    play_audio_channel.single_write(PlayAudioEvent {
                        source: sounds.sound_effects["cash_register_bell"].clone(),
                    });
                }
            }
        }
    }
}
