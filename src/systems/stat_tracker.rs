use crate::{
    components::{HealthComponent, SpaceshipComponent, StoreComponent},
    states::TrackedStats,
};
use amethyst::{
    ecs::prelude::{Join, ReadExpect, ReadStorage, System, WriteStorage},
    ui::UiText,
};

pub struct StatTrackerSystem;

impl<'s> System<'s> for StatTrackerSystem {
    type SystemData = (
        ReadStorage<'s, SpaceshipComponent>,
        ReadStorage<'s, HealthComponent>,
        ReadStorage<'s, StoreComponent>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, TrackedStats>,
    );

    fn run(&mut self, (spaceships, healths, stores, mut ui_text, tracked_stats): Self::SystemData) {
        for spaceship in (&spaceships).join() {
            if let Some(text) = ui_text.get_mut(tracked_stats.currency) {
                text.text = format!("x{}", spaceship.money.to_string());
            }
        }

        for (_spaceship, health) in (&spaceships, &healths).join() {
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
