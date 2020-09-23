use crate::{
    components::{Enemy, Hitbox2DComponent, Motion2DComponent, Spaceship},
    space_shooter::{CollisionEvent, EnemyCollisionEvent, PlayerCollisionEvent},
};
use amethyst::{
    core::{math::Vector2, transform::Transform},
    ecs::*,
    shrev::{EventChannel, ReaderId},
};
use std::collections::HashMap;

#[derive(Default)]
pub struct CollisionDetectionSystem;

/// Detects collisions between entities
impl<'s> System<'s> for CollisionDetectionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Hitbox2DComponent>,
        ReadStorage<'s, Transform>,
        Write<'s, EventChannel<CollisionEvent>>,
    );
    fn run(&mut self, (entities, hitbox2ds, transforms, mut collision_channel): Self::SystemData) {
        for (entity_a, transform_a, hitbox_a) in (&entities, &transforms, &hitbox2ds).join() {
            for (entity_b, transform_b, hitbox_b) in (&entities, &transforms, &hitbox2ds).join() {
                if entity_a == entity_b {
                    continue;
                }

                if hitbox_a.is_colliding(hitbox_b, transform_a, transform_b) {
                    collision_channel.single_write(CollisionEvent::new(entity_a, entity_b));
                }
            }
        }
    }
}

#[derive(Default)]
pub struct CollisionHandlerSystem {
    event_reader: Option<ReaderId<CollisionEvent>>,
}

/// Handles collision events between entities
impl<'s> System<'s> for CollisionHandlerSystem {
    type SystemData = (
        ReadStorage<'s, Spaceship>,
        ReadStorage<'s, Enemy>,
        ReadStorage<'s, Motion2DComponent>,
        Read<'s, EventChannel<CollisionEvent>>,
        Write<'s, EventChannel<PlayerCollisionEvent>>,
        Write<'s, EventChannel<EnemyCollisionEvent>>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<CollisionEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (
            spaceships,
            enemies,
            motions,
            collision_channel,
            mut player_collision_channel,
            mut enemy_collision_channel,
        ): Self::SystemData,
    ) {
        for event in collision_channel.read(self.event_reader.as_mut().unwrap()) {
            let mut collision_velocity: Option<Vector2<f32>> = None;

            if let Some(motion_component) = motions.get(event.entity_b) {
                collision_velocity = Some(motion_component.velocity);
            }

            if let Some(_spaceship) = spaceships.get(event.entity_a) {
                player_collision_channel.single_write(PlayerCollisionEvent::new(
                    event.entity_a,
                    event.entity_b,
                    collision_velocity,
                ));
            } else if let Some(_enemy) = enemies.get(event.entity_a) {
                enemy_collision_channel.single_write(EnemyCollisionEvent::new(
                    event.entity_a,
                    event.entity_b,
                    collision_velocity,
                ));
            }
        }
    }
}
