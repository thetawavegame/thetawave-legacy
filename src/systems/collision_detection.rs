use crate::{
    components::{Enemy, Hitbox2DComponent, Motion2DComponent, Spaceship},
    space_shooter::CollisionEvent,
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
        ReadStorage<'s, Hitbox2DComponent>,
        ReadStorage<'s, Motion2DComponent>,
        ReadStorage<'s, Transform>,
        Entities<'s>,
        Write<'s, EventChannel<CollisionEvent>>,
    );

    fn run(
        &mut self,
        (
            enemies,
            spaceships,
            hitboxes,
            motions,
            transforms,
            entities,
            mut collision_event_channel,
        ): Self::SystemData,
    ) {
        for (enemy, enemy_transform, _enemy, enemy_hitbox, enemy_motion) in
            (&entities, &transforms, &enemies, &hitboxes, &motions).join()
        {
            //check for enemy collisions
            for (enemy_b, enemy_b_transform, _enemy_b, enemy_b_hitbox, enemy_b_motion) in
                (&entities, &transforms, &enemies, &hitboxes, &motions).join()
            {
                if enemy == enemy_b {
                    continue;
                }

                if enemy_hitbox.is_colliding(enemy_b_hitbox, enemy_transform, enemy_b_transform) {
                    collision_event_channel.single_write(CollisionEvent::new(
                        enemy,
                        String::from("enemy"),
                        enemy_motion.velocity.x,
                        enemy_motion.velocity.y,
                        enemy_b,
                        String::from("enemy"),
                        enemy_b_motion.velocity.x,
                        enemy_b_motion.velocity.y,
                    ));
                }
            }

            //check for spaceship collisions
            for (
                spaceship_entity,
                spaceship_transform,
                _spaceship,
                spaceship_hitbox,
                spaceship_motion,
            ) in (&entities, &transforms, &spaceships, &hitboxes, &motions).join()
            {
                if spaceship_hitbox.is_colliding(enemy_hitbox, spaceship_transform, enemy_transform)
                {
                    collision_event_channel.single_write(CollisionEvent::new(
                        enemy,
                        String::from("enemy"),
                        enemy_motion.velocity.x,
                        enemy_motion.velocity.y,
                        spaceship_entity,
                        String::from("spaceship"),
                        spaceship_motion.velocity.x,
                        spaceship_motion.velocity.y,
                    ));
                }
            }
        }
    }
}
