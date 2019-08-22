use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities,Join, ReadStorage, System, WriteStorage, Read, Resources, Write},
    ecs::*,
    shrev::{EventChannel, ReaderId},
    
};


use crate::{
    components::{Enemy},
    systems::hitbox_collide,
    space_shooter::EnemyCollisionEvent,
};

#[derive(Default)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {

    type SystemData = (
        ReadStorage<'s, Enemy>,
        ReadStorage<'s, Transform>,
        Entities<'s>,
        Write<'s, EventChannel<EnemyCollisionEvent>>,
    );

    fn run(&mut self, (enemies, transforms, entities, mut enemy_collision_event_channel): Self::SystemData) {

        for (entity_a, transform_a, enemy_a) in (&entities, &transforms, &enemies).join() {
            for (entity_b, transform_b, enemy_b) in (&entities, &transforms, &enemies).join() {
                if entity_a == entity_b {
                    continue;
                }

                if hitbox_collide(transform_a.translation().x, transform_a.translation().y, transform_b.translation().x, transform_b.translation().y, enemy_a.hitbox_width, enemy_a.hitbox_height, enemy_b.hitbox_width, enemy_b.hitbox_height) {
                    enemy_collision_event_channel.single_write(EnemyCollisionEvent::new(entity_a, entity_b));
                }
            }
        }
    }
}