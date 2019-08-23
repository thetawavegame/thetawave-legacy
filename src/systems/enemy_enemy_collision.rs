use amethyst::{
    ecs::prelude::{System, Read, Resources},
    ecs::*,
    shrev::{EventChannel, ReaderId},
    
};


use crate::{
    space_shooter::CollisionEvent,
};

#[derive(Default)]
pub struct EnemyEnemyCollisionSystem{
    event_reader: Option<ReaderId<CollisionEvent>>,
}

impl<'s> System<'s> for EnemyEnemyCollisionSystem {

    type SystemData = (
        Read<'s, EventChannel<CollisionEvent>>
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.event_reader = Some(res.fetch_mut::<EventChannel<CollisionEvent>>().register_reader());
    }

    fn run(&mut self, enemy_collision_event_channel: Self::SystemData) {

        for event in enemy_collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            println!("Recieved an event: {:?}", event);
        }
        
    }
}