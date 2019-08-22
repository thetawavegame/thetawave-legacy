use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities,Join, ReadStorage, System, WriteStorage, Read, Resources},
    ecs::*,
    shrev::{EventChannel, ReaderId},
    
};


use crate::{
    components::{Enemy},
    systems::hitbox_collide,
    space_shooter::EnemyCollisionEvent,
};

#[derive(Default)]
pub struct EnemyEnemyCollisionSystem{
    event_reader: Option<ReaderId<EnemyCollisionEvent>>,
}

impl<'s> System<'s> for EnemyEnemyCollisionSystem {

    type SystemData = (
        WriteStorage<'s, Enemy>,
        ReadStorage<'s, Transform>,
        Read<'s, EventChannel<EnemyCollisionEvent>>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.event_reader = Some(res.fetch_mut::<EventChannel<EnemyCollisionEvent>>().register_reader());
    }

    fn run(&mut self, (mut enemies, transforms, enemy_collision_event_channel): Self::SystemData) {

        for event in enemy_collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            println!("Recieved an event: {:?}", event);
        }
        /*
        for (enemy, enemy_transform) in (&mut enemies, &transforms).join() {
            if let Some(enemy) = prev_enemy {
                println!("prev enemy: {}", enemy.damage);
            }
            prev_enemy = enemy;
        }
        */
    }
}