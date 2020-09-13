use crate::{
    audio::{play_sfx, Sounds},
    components::{Enemy, Motion2DComponent, Spaceship},
    space_shooter::CollisionEvent,
};
use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    ecs::*,
    ecs::{Read, System, World},
    shrev::{EventChannel, ReaderId},
};

/// EnemyCollisionSystem updates enemy data when a collision is detected for
/// enemy entities.
#[derive(Default)]
pub struct EnemyCollisionSystem {
    event_reader: Option<ReaderId<CollisionEvent>>,
}

impl<'s> System<'s> for EnemyCollisionSystem {
    type SystemData = (
        Read<'s, EventChannel<CollisionEvent>>,
        WriteStorage<'s, Spaceship>,
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Motion2DComponent>,
        Entities<'s>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
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
            mut spaceships,
            mut enemies,
            mut motions,
            entities,
            storage,
            sounds,
            audio_output,
        ): Self::SystemData,
    ) {
        for event in enemy_collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            play_sfx(&sounds.crash_sfx, &storage, audio_output.as_deref());

            for spaceship_component in (&mut spaceships).join() {
                for (enemy, enemy_motion, enemy_entity) in
                    (&mut enemies, &mut motions, &entities).join()
                {
                    if event.type_b == "enemy" && event.type_a == "enemy" {
                        if event.entity_a == enemy_entity || event.entity_b == enemy_entity {
                            if event.entity_a == enemy_entity
                                && enemy.name != "repeater_body"
                                && enemy.name != "repeater_head"
                            {
                                enemy.health -= spaceship_component.collision_damage;
                                enemy_motion.velocity.x = event.to_velocity_x_a;
                                enemy_motion.velocity.y = event.to_velocity_y_a;
                            } else if event.entity_b == enemy_entity
                                && enemy.name != "repeater_body"
                                && enemy.name != "repeater_head"
                            {
                                enemy.health -= spaceship_component.collision_damage;
                                enemy_motion.velocity.x = event.to_velocity_x_b;
                                enemy_motion.velocity.y = event.to_velocity_y_b;
                            }
                        }
                    } else if event.type_b == "spaceship"
                        && event.type_a == "enemy"
                        && event.entity_a == enemy_entity
                    {
                        if enemy.name != "repeater_body"
                            && enemy.name != "repeater_head"
                            && enemy.name != "repeater_right_shoulder"
                            && enemy.name != "repeater_left_shoulder"
                            && enemy.name != "repeater_right_arm"
                            && enemy.name != "repeater_left_arm"
                        {
                            enemy.health -= spaceship_component.collision_damage;
                            enemy_motion.velocity.x =
                                (-(1.0) * event.to_velocity_x_a) + event.to_velocity_x_b;
                            enemy_motion.velocity.y =
                                (-(1.0) * event.to_velocity_y_a) + event.to_velocity_y_b;
                        }
                    }
                }
            }
        }
    }
}
