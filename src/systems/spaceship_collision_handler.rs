use crate::{
    components::{Enemy, Motion2DComponent, Spaceship},
    events::CollisionEvent,
};
use amethyst::{
    ecs::*,
    ecs::{Read, System, World},
    shrev::{EventChannel, ReaderId},
};

/// SpaceshipCollisionSystem handles updating player data when a collision is
/// detected.
#[derive(Default)]
pub struct SpaceshipCollisionSystem {
    event_reader: Option<ReaderId<CollisionEvent>>,
}

impl<'s> System<'s> for SpaceshipCollisionSystem {
    type SystemData = (
        Read<'s, EventChannel<CollisionEvent>>,
        ReadStorage<'s, Enemy>,
        WriteStorage<'s, Spaceship>,
        WriteStorage<'s, Motion2DComponent>,
        Entities<'s>,
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
          enemy_collision_event_channel,
          enemies,
          mut spaceships,
          mut motions,
          entities,
      ): Self::SystemData,
    ) {
        for event in enemy_collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            for (spaceship, motion) in (&mut spaceships, &mut motions).join() {
                for (enemy, enemy_entity) in (&enemies, &entities).join() {
                    if event.type_b == "spaceship"
                        && event.type_a == "enemy"
                        && event.entity_a == enemy_entity
                    {
                        if spaceship.barrel_action_left {
                            spaceship.barrel_action_right = true;
                            spaceship.barrel_action_left = false;
                        } else if spaceship.barrel_action_right {
                            spaceship.barrel_action_left = true;
                            spaceship.barrel_action_right = false;
                        }

                        let enemy_dead = enemy.health <= 0.0;

                        if (!spaceship.steel_barrel && !enemy_dead)
                            || (!spaceship.barrel_action_left && !spaceship.barrel_action_right)
                        {
                            spaceship.health -= enemy.collision_damage;
                        }

                        // Push the ship in the opposite direction.
                        motion.velocity.x =
                            (-(1.0) * event.to_velocity_x_b) + event.to_velocity_x_a;

                        motion.velocity.y =
                            (-(1.0) * event.to_velocity_y_b) + event.to_velocity_y_a;
                    }
                }
            }
        }
    }
}
