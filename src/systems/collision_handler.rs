use amethyst::{
    ecs::{System, Read, World},
    ecs::*,
    shrev::{EventChannel, ReaderId},
    audio::{output::Output, Source},
    assets::AssetStorage,
};
use std::ops::Deref;
use crate::{
    space_shooter::CollisionEvent,
    components::{Spaceship, Enemy},
    audio::{play_sfx, Sounds},
};

#[derive(Default)]
pub struct CollisionHandlerSystem{
    event_reader: Option<ReaderId<CollisionEvent>>,
}

impl<'s> System<'s> for CollisionHandlerSystem {

    type SystemData = (
        Read<'s, EventChannel<CollisionEvent>>,
        WriteStorage<'s, Spaceship>,
        WriteStorage<'s, Enemy>,
        Entities<'s>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(world.fetch_mut::<EventChannel<CollisionEvent>>().register_reader());
    }

    fn run(&mut self, (enemy_collision_event_channel, mut spaceships, mut enemies, entities,storage, sounds, audio_output ): Self::SystemData) {

        for event in enemy_collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            //println!("{:?}", event);
            play_sfx(&sounds.crash_sfx, &storage, audio_output.as_ref().map(|o| o.deref()));
            
            for spaceship in (&mut spaceships).join() {
                for (enemy, enemy_entity) in (&mut enemies, &entities).join() {
                    if event.type_b == "enemy" && event.type_a == "enemy" {
                        if event.entity_a == enemy_entity ||  event.entity_b == enemy_entity {

                            if event.entity_a == enemy_entity && enemy.name != "repeater_body" {
                                enemy.health -= spaceship.collision_damage;
                                enemy.current_velocity_x = event.to_velocity_x_a;
                                enemy.current_velocity_y = event.to_velocity_y_a;
                            } else if event.entity_b == enemy_entity && enemy.name != "repeater_body" {
                                enemy.health -= spaceship.collision_damage;
                                enemy.current_velocity_x = event.to_velocity_x_b;
                                enemy.current_velocity_y = event.to_velocity_y_b;
                            }

                        }
                    } else if event.type_b == "spaceship" && event.type_a == "enemy" {
                        if event.entity_a == enemy_entity {

                        
                            //println!("spaceship collision");
                            let mut enemy_dead = false;
                            if enemy.health <= 0.0 {
                                enemy_dead = true;
                            }

                            if spaceship.barrel_action_left {
                                spaceship.barrel_action_right = true;
                                spaceship.barrel_action_left = false;

                            }else if spaceship.barrel_action_right {
                                spaceship.barrel_action_left = true;
                                spaceship.barrel_action_right = false;

                            }

                            if !spaceship.steel_barrel && !enemy_dead {
                                spaceship.health -= enemy.collision_damage;
                            }else if !spaceship.barrel_action_left && !spaceship.barrel_action_right{
                                spaceship.health -= enemy.collision_damage;
                            }

                            let temp_velocity_x = spaceship.current_velocity_x;
                            spaceship.current_velocity_x = (-(1.0) * spaceship.current_velocity_x) + enemy.current_velocity_x;

                            let temp_velocity_y = spaceship.current_velocity_y;
                            spaceship.current_velocity_y = (-(1.0) * spaceship.current_velocity_y) + enemy.current_velocity_y;

                            if enemy.name != "repeater_body" {
                                enemy.health -= spaceship.collision_damage;
                                enemy.current_velocity_x = (-(1.0) * enemy.current_velocity_x) + temp_velocity_x;
                                enemy.current_velocity_y = (-(1.0) * enemy.current_velocity_y) + temp_velocity_y;
                            }
                        }
                    }

                }
            }

        }
    }
}