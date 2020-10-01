use crate::{
    audio::Sounds,
    components::{Spaceship, Store},
    events::PlayAudioEvent,
    resources::{ItemPool, SpriteResource},
};
use amethyst::{
    core::timing::Time,
    ecs::{Entities, Join, LazyUpdate, Read, ReadExpect, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    shrev::EventChannel,
};

pub struct StoreSystem;

impl<'s> System<'s> for StoreSystem {
    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, ItemPool>,
        WriteStorage<'s, Store>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, Spaceship>,
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
            mut spaceships,
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

            for spaceship in (&mut spaceships).join() {
                if (buy_0_action
                    && store.purchase_item(0, &entities, spaceship, &sprite_resource, &lazy_update))
                    || (buy_1_action
                        && store.purchase_item(
                            1,
                            &entities,
                            spaceship,
                            &sprite_resource,
                            &lazy_update,
                        ))
                    || (buy_2_action
                        && store.purchase_item(
                            2,
                            &entities,
                            spaceship,
                            &sprite_resource,
                            &lazy_update,
                        ))
                {
                    play_audio_channel.single_write(PlayAudioEvent {
                        source: sounds.cash_register_bell_sfx.clone(),
                    });
                }
            }
        }
    }
}
