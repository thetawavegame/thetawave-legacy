use amethyst::{
    ecs::prelude::{Entities, Join, System, WriteStorage, ReadStorage, ReadExpect},
    ui::UiText,
};

use crate::{
    components::{Spaceship},
    space_shooter::TrackedStats,
};


pub struct StatTrackerSystem;
impl<'s> System<'s> for StatTrackerSystem {

    type SystemData = (
        ReadStorage<'s, Spaceship>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, TrackedStats>,
    );

    fn run(&mut self, (spaceships, mut ui_text, tracked_stats): Self::SystemData) {

        for (spaceship) in (&spaceships).join() {
            if let Some(text) = ui_text.get_mut(tracked_stats.currency) {
                //text.text = spaceship.money.to_string();
                text.text = format!("x {}", spaceship.money.to_string());
            }

        }

    }
}
