use crate::{
    audio::Sounds,
    entities::SpawnableType,
    events::PlayAudioEvent,
    player::components::PlayerComponent,
    spawnable::resources::{ConsumablesResource, ItemsResource},
    store::resources::StoreResource,
    visual::components::StoreIconComponent,
    visual::resources::SpriteSheetsResource,
};
use amethyst::{
    core::{timing::Time, Transform},
    ecs::{
        Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, Write, WriteExpect,
        WriteStorage,
    },
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
    shrev::EventChannel,
};

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
        ReadStorage<'s, StoreIconComponent>,
        WriteStorage<'s, SpriteRender>,
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
            store_icons,
            mut sprite_renders,
        ): Self::SystemData,
    ) {
        let buy_0_action = input.action_is_down("buy_0").unwrap();
        let buy_1_action = input.action_is_down("buy_1").unwrap();
        let buy_2_action = input.action_is_down("buy_2").unwrap();

        if store_resource.restock_when_ready(time.delta_seconds()) {
            // change store icons
            for (store_icon, sprite_render) in (&store_icons, &mut sprite_renders).join() {
                if let Some(SpawnableType::Item(item_type)) =
                    store_resource.inventory[store_icon.inventory_index].clone()
                {
                    let item_data = items_resource.item_entities[&item_type].clone();
                    *sprite_render = SpriteRender {
                        sprite_sheet: spritesheets_resource.spritesheets
                            [&item_data.sprite_render_data.spritesheet]
                            .clone(),
                        sprite_number: item_data.sprite_render_data.initial_index,
                    }
                } else if let Some(SpawnableType::Consumable(consumable_type)) =
                    store_resource.inventory[store_icon.inventory_index].clone()
                {
                    let consumable_data =
                        consumables_resource.consumable_entities[&consumable_type].clone();
                    *sprite_render = SpriteRender {
                        sprite_sheet: spritesheets_resource.spritesheets
                            [&consumable_data.sprite_render_data.spritesheet]
                            .clone(),
                        sprite_number: consumable_data.sprite_render_data.initial_index,
                    }
                } else {
                    // if no inventory in slot put blank sprite
                    *sprite_render = SpriteRender {
                        sprite_sheet: spritesheets_resource.spritesheets["items"].clone(),
                        sprite_number: 0,
                    }
                }
            }
        }

        // TODO: streamline purchase_item function with constant component in item data file
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
                play_audio_channel.single_write(PlayAudioEvent {
                    source: sounds.sound_effects["cash_register_bell"].clone(),
                });

                // change store icons
                for (store_icon, sprite_render) in (&store_icons, &mut sprite_renders).join() {
                    if buy_0_action && store_icon.inventory_index == 0 {
                        *sprite_render = SpriteRender {
                            sprite_sheet: spritesheets_resource.spritesheets["items"].clone(),
                            sprite_number: 0,
                        }
                    }
                    if buy_1_action && store_icon.inventory_index == 1 {
                        *sprite_render = SpriteRender {
                            sprite_sheet: spritesheets_resource.spritesheets["items"].clone(),
                            sprite_number: 0,
                        }
                    }
                    if buy_2_action && store_icon.inventory_index == 2 {
                        *sprite_render = SpriteRender {
                            sprite_sheet: spritesheets_resource.spritesheets["items"].clone(),
                            sprite_number: 0,
                        }
                    }
                }
            }
        }
    }
}
