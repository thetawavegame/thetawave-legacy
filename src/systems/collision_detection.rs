use crate::{
    components::{Enemy, Spaceship},
    space_shooter::CollisionEvent,
    systems::hitbox_collide,
};
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities, Join, ReadStorage, System, Write},
    shrev::EventChannel,
};

#[derive(Default)]
pub struct CollisionDetectionSystem;

impl<'s> System<'s> for CollisionDetectionSystem {
    type SystemData = (
        ReadStorage<'s, Enemy>,
        ReadStorage<'s, Spaceship>,
        ReadStorage<'s, Transform>,
        Entities<'s>,
        Write<'s, EventChannel<CollisionEvent>>,
    );

    fn run(
        &mut self,
        (enemies, spaceships, transforms, entities, mut enemy_collision_event_channel): Self::SystemData,
    ) {
        for (entity_a, transform_a, enemy_a) in (&entities, &transforms, &enemies).join() {
            //check for enemy collisions
            for (entity_b, transform_b, enemy_b) in (&entities, &transforms, &enemies).join() {
                if entity_a == entity_b {
                    continue;
                }

                if hitbox_collide(
                    transform_a.translation().x,
                    transform_a.translation().y,
                    transform_b.translation().x,
                    transform_b.translation().y,
                    enemy_a.hitbox_width,
                    enemy_a.hitbox_height,
                    enemy_b.hitbox_width,
                    enemy_b.hitbox_height,
                    enemy_a.hitbox_x_offset,
                    enemy_a.hitbox_y_offset,
                    enemy_b.hitbox_x_offset,
                    enemy_b.hitbox_y_offset,
                ) {
                    enemy_collision_event_channel.single_write(CollisionEvent::new(
                        entity_a,
                        String::from("enemy"),
                        enemy_b.current_velocity_x,
                        enemy_b.current_velocity_y,
                        entity_b,
                        String::from("enemy"),
                        enemy_a.current_velocity_x,
                        enemy_a.current_velocity_y,
                    ));
                }
            }

            //check for spaceship collisions
            for (entity_b, transform_b, spaceship_b) in (&entities, &transforms, &spaceships).join()
            {
                if hitbox_collide(
                    transform_a.translation().x,
                    transform_a.translation().y,
                    transform_b.translation().x,
                    transform_b.translation().y,
                    enemy_a.hitbox_width,
                    enemy_a.hitbox_height,
                    spaceship_b.hitbox_width,
                    spaceship_b.hitbox_height,
                    enemy_a.hitbox_x_offset,
                    enemy_a.hitbox_y_offset,
                    spaceship_b.hitbox_x_offset,
                    spaceship_b.hitbox_y_offset,
                ) {
                    enemy_collision_event_channel.single_write(CollisionEvent::new(
                        entity_a,
                        String::from("enemy"),
                        spaceship_b.current_velocity_x,
                        spaceship_b.current_velocity_y,
                        entity_b,
                        String::from("spaceship"),
                        enemy_a.current_velocity_x,
                        enemy_a.current_velocity_y,
                    ));
                }
            }
        }
    }
}
