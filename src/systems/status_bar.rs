use amethyst::{
    core::{
        nalgebra::Vector3,
    },
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
                        let status_divisor = spaceship.max_health / status_bar.unit_limit;
                        let mut status_unit_num = ((spaceship.health) / status_divisor).ceil() as usize;
                        if spaceship.health <= 0.0 {
                            status_unit_num = 0;
                        }

                        //push units on
                        if status_unit_num > status_bar.status_unit_stack.len() {
                            let status_position = Vector3::new(
                                status_bar.x_pos, status_bar.y_pos, Z,
                            );
                            status_bar.y_pos += 1.0;
                            status_bar.status_unit_stack.push(spawn_status_unit(&entities, &sprite_resource, HEALTH_SPRITE_INDEX, status_position, &lazy_update));
                        }

                        if status_unit_num < status_bar.status_unit_stack.len() {
                            if let Some(unit) = status_bar.status_unit_stack.pop() {
                                let _result = entities.delete(unit);
                                status_bar.y_pos -= 1.0;
                            }
                        }
                    }
                }

                StatusType::Defense => {
                    for defense in (&mut defenses).join() {
                        //ensure that defense stays between 0 and max defense
                        if defense.defense < 0.0 {
                            defense.defense = 0.0;
                        } else if defense.defense > defense.max_defense {
                            defense.defense = defense.max_defense;
                        }

                        //find the number of points per unit in the bar and the number of units needed
                        let status_divisor = defense.max_defense / status_bar.unit_limit;
                        let status_unit_num = ((defense.defense) / status_divisor).ceil() as usize;


                        //push units onto stack if needed
                        if status_unit_num > status_bar.status_unit_stack.len() {
                            let status_position = Vector3::new(
                                status_bar.x_pos, status_bar.y_pos, Z,
                            );
                            status_bar.y_pos += 1.0;
                            status_bar.status_unit_stack.push(spawn_status_unit(&entities, &sprite_resource, DEFENSE_SPRITE_INDEX, status_position, &lazy_update));
                        }

                        //delete units from stack if needed
                        if status_unit_num < status_bar.status_unit_stack.len() {
                            if let Some(unit) = status_bar.status_unit_stack.pop() {
                                let _result = entities.delete(unit);
                                status_bar.y_pos -= 1.0;
                            }
                        }
                    }
                }


                StatusType::Roll => {
                    for spaceship in (&spaceships).join() {
                        let status_divisor = spaceship.barrel_cooldown/status_bar.unit_limit;
                        let mut status_unit_num = ((spaceship.barrel_cooldown - spaceship.barrel_reset_timer) / status_divisor).ceil() as usize;
                        if spaceship.barrel_action_left || spaceship.barrel_action_right {
                            status_unit_num = 0;
                        }

                        //push units on
                        if status_unit_num > status_bar.status_unit_stack.len() {
                            let status_position = Vector3::new(
                                status_bar.x_pos, status_bar.y_pos, Z,
                            );
                            status_bar.x_pos += 1.0;
                            status_bar.status_unit_stack.push(spawn_status_unit(&entities, &sprite_resource, ROLL_SPRITE_INDEX, status_position, &lazy_update));
                        }

                        if status_unit_num < status_bar.status_unit_stack.len() {
                            if let Some(unit) = status_bar.status_unit_stack.pop() {
                                status_bar.x_pos -= 1.0;
                                let _result = entities.delete(unit);
                            }

                        }
                    }
                }

            }
        }
    }
}