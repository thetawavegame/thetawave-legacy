use crate::{
    components::{HealthComponent, PlayerComponent, StoreComponent},
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
        ReadStorage<'s, StoreComponent>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, TrackedStats>,
    );

    fn run(&mut self, (players, healths, stores, mut ui_text, tracked_stats): Self::SystemData) {
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

        for store in (&stores).join() {
            if let Some(text) = ui_text.get_mut(tracked_stats.item_price_1) {
                if let Some(item) = &store.item_inventory[0] {
                    text.text = format!("${}", item.item_component.price);
                } else {
                    text.text = "$0".to_string();
                }
            }
            if let Some(text) = ui_text.get_mut(tracked_stats.item_price_2) {
                if let Some(item) = &store.item_inventory[1] {
                    text.text = format!("${}", item.item_component.price);
                } else {
                    text.text = "$0".to_string();
                }
            }
            if let Some(text) = ui_text.get_mut(tracked_stats.item_price_3) {
                if let Some(item) = &store.item_inventory[2] {
                    text.text = format!("${}", item.item_component.price);
                } else {
                    text.text = "$0".to_string();
                }
            }
        }
    }
}
