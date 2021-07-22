use crate::{
    audio::Sounds,
    events::PlayAudioEvent,
    player::PlayerComponent,
    spawnable::{ConsumablesResource, ItemsResource},
    store::StoreResource,
    visual::SpriteSheetsResource,
};
use amethyst::{
    core::{timing::Time, Transform},
    ecs::{
        Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, Write, WriteExpect,
        WriteStorage,
    },
    input::{InputHandler, StringBindings},
    shrev::EventChannel,
};

/// Handles store resource
/// Updates and handles purchases by players
pub struct StoreSystem;

impl<'s> System<'s> for StoreSystem {
    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, SpriteSheetsResource>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, ItemsResource>,
        ReadExpect<'s, ConsumablesResource>,
        WriteExpect<'s, StoreResource>,
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
            spritesheets_resource,
            lazy_update,
            items_resource,
            consumables_resource,
            mut store_resource,
            time,
            input,
            mut players,
            transforms,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        // check for input
        let buy_0_action = input.action_is_down("buy_0").unwrap();
        let buy_1_action = input.action_is_down("buy_1").unwrap();
        let buy_2_action = input.action_is_down("buy_2").unwrap();

        // update store
        store_resource.update(
            time.delta_seconds(),
            &spritesheets_resource,
            &items_resource,
            &consumables_resource,
            &entities,
            &lazy_update,
        );

        // purchase store inventory on input if available
        for (character, transform) in (&mut players, &transforms).join() {
            if (buy_0_action
                && store_resource.purchase(
                    0,
                    &entities,
                    character,
                    transform,
                    &items_resource,
                    &consumables_resource,
                    &spritesheets_resource,
                    &lazy_update,
                ))
                || (buy_1_action
                    && store_resource.purchase(
                        1,
                        &entities,
                        character,
                        transform,
                        &items_resource,
                        &consumables_resource,
                        &spritesheets_resource,
                        &lazy_update,
                    ))
                || (buy_2_action
                    && store_resource.purchase(
                        2,
                        &entities,
                        character,
                        transform,
                        &items_resource,
                        &consumables_resource,
                        &spritesheets_resource,
                        &lazy_update,
                    ))
            {
                // play purchase sound effect
                play_audio_channel.single_write(PlayAudioEvent {
                    source: sounds.sound_effects["cash_register_bell"].clone(),
                });
            }
        }
    }
}
