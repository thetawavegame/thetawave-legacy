use amethyst::{
    ecs::prelude::{Join, System, WriteStorage, ReadStorage, ReadExpect},
    ui::UiText,
};

use crate::{
    components::{Spaceship, Store},
    space_shooter::TrackedStats,
};


pub struct StatTrackerSystem;
impl<'s> System<'s> for StatTrackerSystem {

    type SystemData = (
        ReadStorage<'s, Spaceship>,
        ReadStorage<'s, Store>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, TrackedStats>,
    );

    fn run(&mut self, (spaceships, stores, mut ui_text, tracked_stats): Self::SystemData) {

        for spaceship in (&spaceships).join() {
            if let Some(text) = ui_text.get_mut(tracked_stats.currency) {
                //text.text = spaceship.money.to_string();
                text.text = format!("x {}", spaceship.money.to_string());
            }

        }

        for store in (&stores).join() {
            if let Some(text) = ui_text.get_mut(tracked_stats.item_price_1) {
                if let Some(item) = &store.item_inventory[0] {
                    text.text = format!("₹{}", item.price);
                }
            }
            if let Some(text) = ui_text.get_mut(tracked_stats.item_price_2) {
                if let Some(item) = &store.item_inventory[1] {
                    text.text = format!("₹{}", item.price);
                }
            }
            if let Some(text) = ui_text.get_mut(tracked_stats.item_price_3) {
                if let Some(item) = &store.item_inventory[2] {
                    text.text = format!("₹{}", item.price);
                }
            }
        }

    }
}
