use amethyst::{
    ecs::prelude::{System, Read, Resources},
    ecs::*,
    shrev::{EventChannel, ReaderId},
    core::transform::Transform,
    
};


use crate::{
    space_shooter::CollisionEvent,
    components::{Spaceship, Enemy},
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
        ReadStorage<'s, Transform>,
        Entities<'s>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.event_reader = Some(res.fetch_mut::<EventChannel<CollisionEvent>>().register_reader());
    }

    fn run(&mut self, (enemy_collision_event_channel, mut spaceships, mut enemies, transforms, entities): Self::SystemData) {

        for event in enemy_collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            println!("{:?}", event);
            
            for (spaceship, spaceship_transform, spaceship_entity) in (&mut spaceships, &transforms, &entities).join() {
                for (enemy, enemy_transform, enemy_entity) in (&mut enemies, &transforms, &entities).join() {
                    if event.type_b == "enemy" && event.type_a == "enemy" {
                        if event.entity_a == enemy_entity ||  event.entity_b == enemy_entity {

                            if event.entity_a == enemy_entity {
                                enemy.health -= spaceship.collision_damage;
                                enemy.current_velocity_x = event.to_velocity_x_a;
                                enemy.current_velocity_y = event.to_velocity_y_a;
                            } else if event.entity_b == enemy_entity {
                                enemy.health -= spaceship.collision_damage;
                                enemy.current_velocity_x = event.to_velocity_x_b;
                                enemy.current_velocity_y = event.to_velocity_y_b;
                            }

                        }
                    }

                    /*
                    else if event.type_b == "spaceship" {

                    }
                    */
                }
            }

        }
        /*
        for event in enemy_collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            //println!("Recieved an event: {:?}", event);
            if event.type_b == "enemy"{
                println!("enemy to enemy collision");
                //handle enemy to enemy collision
                for (enemy, enemy_transform, enemy_entity) in (&mut enemies, &transforms, &entities).join() {
                    if enemy_entity == event.entity_a || enemy_entity == event.entity_b {
                        
                    }
                }
            }else if event.type_b == "spaceship"{
                println!("enemy to spaceship collision");
                //handle spaceship collision
                for (spaceship, spaceship_transform) in (&mut spaceships, &transforms).join() {
                    for (enemy, enemy_transform) in (&mut enemies, &transforms).join() {

                    }
                }
            }

        }
        */
        
    }
}