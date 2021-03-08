use crate::{
    components::{HealthComponent, PlayerComponent},
    entities::EntityType,
    resources::{ConsumablesResource, ItemsResource, StoreResource},
    states::TrackedStats,
};
use amethyst::{
    ecs::prelude::{Join, ReadExpect, ReadStorage, System, WriteStorage},
    ui::UiText,
};

pub struct StatTrackerSystem;

impl<'s> System<'s> for StatTrackerSystem {
    type SystemData = (
        ReadStorage<'s, PlayerComponent>,
        ReadStorage<'s, HealthComponent>,
        ReadExpect<'s, StoreResource>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, TrackedStats>,
        ReadExpect<'s, ItemsResource>,
        ReadExpect<'s, ConsumablesResource>,
    );

    fn run(
        &mut self,
        (
            players,
            healths,
            store_resource,
            mut ui_text,
            tracked_stats,
            items_resource,
            consumables_resource,
        ): Self::SystemData,
    ) {
        for character in (&players).join() {
            if let Some(text) = ui_text.get_mut(tracked_stats.currency) {
                text.text = format!("x{}", character.money.to_string());
            }
        }

        for (_player, health) in (&players, &healths).join() {
            if let Some(text) = ui_text.get_mut(tracked_stats.shields) {
                text.text = format!("x{}", health.armor.to_string());
            }
        }

        if let Some(text) = ui_text.get_mut(tracked_stats.item_price_1) {
            if let Some(EntityType::Item(item_type)) = &store_resource.inventory[0] {
                text.text = format!(
                    "${}",
                    items_resource.item_entities[item_type].item_component.price
                );
            } else if let Some(EntityType::Consumable(consumable_type)) =
                &store_resource.inventory[0]
            {
                text.text = format!(
                    "${}",
                    consumables_resource.consumable_entities[consumable_type]
                        .consumable_component
                        .price
                );
            } else {
                text.text = "$0".to_string();
            }
        }

        if let Some(text) = ui_text.get_mut(tracked_stats.item_price_2) {
            if let Some(EntityType::Item(item_type)) = &store_resource.inventory[1] {
                text.text = format!(
                    "${}",
                    items_resource.item_entities[item_type].item_component.price
                );
            } else if let Some(EntityType::Consumable(consumable_type)) =
                &store_resource.inventory[0]
            {
                text.text = format!(
                    "${}",
                    consumables_resource.consumable_entities[consumable_type]
                        .consumable_component
                        .price
                );
            } else {
                text.text = "$0".to_string();
            }
        }

        if let Some(text) = ui_text.get_mut(tracked_stats.item_price_3) {
            if let Some(EntityType::Item(item_type)) = &store_resource.inventory[2] {
                text.text = format!(
                    "${}",
                    items_resource.item_entities[item_type].item_component.price
                );
            } else if let Some(EntityType::Consumable(consumable_type)) =
                &store_resource.inventory[0]
            {
                text.text = format!(
                    "${}",
                    consumables_resource.consumable_entities[consumable_type]
                        .consumable_component
                        .price
                );
            } else {
                text.text = "$0".to_string();
            }
        }
    }
}
