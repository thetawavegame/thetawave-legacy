use amethyst::{
    ecs::prelude::{Entities, Join, System, WriteStorage, ReadStorage, ReadExpect, LazyUpdate},
};

use crate::{
    components::{Spaceship, StatusBar, StatusType, Defense},
    entities::spawn_status_unit,
    resources::SpriteResource,
};


const Z: f32 = 0.9;
const HEALTH_SPRITE_INDEX: usize = 10;
const DEFENSE_SPRITE_INDEX: usize = 11;
const ROLL_SPRITE_INDEX: usize = 12;


pub struct StatusBarSystem;
impl<'s> System<'s> for StatusBarSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, StatusBar>,
        ReadStorage<'s, Spaceship>,
        WriteStorage<'s, Defense>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut status_bars, spaceships, mut defenses, sprite_resource, lazy_update): Self::SystemData) {
        for status_bar in (&mut status_bars).join() {

            match status_bar.status_type {

                 StatusType::Health => {
                    for spaceship in (&spaceships).join() {
                        if let Some(status_position) = status_bar.update_units_y(spaceship.max_health, spaceship.health, &entities) {
                            status_bar.status_unit_stack.push(spawn_status_unit(&entities, &sprite_resource, HEALTH_SPRITE_INDEX, status_position, &lazy_update));
                        }
                    }
                }

                StatusType::Defense => {
                    for defense in (&mut defenses).join() {
                        if let Some(status_position) = status_bar.update_units_y(defense.max_defense, defense.defense, &entities) {
                            status_bar.status_unit_stack.push(spawn_status_unit(&entities, &sprite_resource, DEFENSE_SPRITE_INDEX, status_position, &lazy_update));
                        }
                    }
                }

                StatusType::Roll => {
                    for spaceship in (&spaceships).join() {
                        if let Some(status_position) = status_bar.update_units_x(spaceship.barrel_cooldown, (spaceship.barrel_cooldown - spaceship.barrel_reset_timer), &entities) {
                            status_bar.status_unit_stack.push(spawn_status_unit(&entities, &sprite_resource, ROLL_SPRITE_INDEX, status_position, &lazy_update));
                        }
                    }
                }
            }
        }
    }
}