use crate::{
    audio::{play_sfx, Sounds},
    components::{Enemy, Spaceship, Motion2DComponent},
    space_shooter::CollisionEvent,
};
use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    ecs::*,
    ecs::{Read, System, World},
    shrev::{EventChannel, ReaderId},
};

#[derive(Default)]
pub struct CollisionHandlerSystem {
    event_reader: Option<ReaderId<CollisionEvent>>,
}

impl<'s> System<'s> for CollisionHandlerSystem {
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
            mut motion_2d_components,
            entities,
            storage,
            sounds,
            audio_output,
        ): Self::SystemData,
    ) {
        for event in enemy_collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            //println!("{:?}", event);
            play_sfx(&sounds.crash_sfx, &storage, audio_output.as_deref());

            for (spaceship, motion_2d) in (&mut spaceships, &mut motion_2d_components).join() {
                for (enemy, enemy_entity) in (&mut enemies, &entities).join() {
                    if event.type_b == "enemy" && event.type_a == "enemy" {
                        if event.entity_a == enemy_entity || event.entity_b == enemy_entity {
                            if event.entity_a == enemy_entity
                                && enemy.name != "repeater_body"
                                && enemy.name != "repeater_head"
                            {
                                enemy.health -= spaceship.collision_damage;
                                enemy.current_velocity_x = event.to_velocity_x_a;
                                enemy.current_velocity_y = event.to_velocity_y_a;
                            } else if event.entity_b == enemy_entity
                                && enemy.name != "repeater_body"
                                && enemy.name != "repeater_head"
                            {
                                enemy.health -= spaceship.collision_damage;
                                enemy.current_velocity_x = event.to_velocity_x_b;
                                enemy.current_velocity_y = event.to_velocity_y_b;
                            }
                        }
                    } else if event.type_b == "spaceship"
                        && event.type_a == "enemy"
                        && event.entity_a == enemy_entity
                    {
                        //println!("spaceship collision");
                        let enemy_dead = enemy.health <= 0.0;

                        if spaceship.barrel_action_left {
                            spaceship.barrel_action_right = true;
                            spaceship.barrel_action_left = false;
                        } else if spaceship.barrel_action_right {
                            spaceship.barrel_action_left = true;
                            spaceship.barrel_action_right = false;
                        }

                        if (!spaceship.steel_barrel && !enemy_dead)
                            || (!spaceship.barrel_action_left && !spaceship.barrel_action_right)
                        {
                            spaceship.health -= enemy.collision_damage;
                        }

                        let temp_velocity_x = motion_2d.velocity.x;
                        motion_2d.velocity.x =
                            (-(1.0) * temp_velocity_x) + enemy.current_velocity_x;

                        let temp_velocity_y = motion_2d.velocity.y;
                        motion_2d.velocity.y =
                            (-(1.0) * temp_velocity_y) + enemy.current_velocity_y;

                        if enemy.name != "repeater_body"
                            && enemy.name != "repeater_head"
                            && enemy.name != "repeater_right_shoulder"
                            && enemy.name != "repeater_left_shoulder"
                            && enemy.name != "repeater_right_arm"
                            && enemy.name != "repeater_left_arm"
                        {
                            enemy.health -= spaceship.collision_damage;
                            enemy.current_velocity_x =
                                (-(1.0) * enemy.current_velocity_x) + temp_velocity_x;
                            enemy.current_velocity_y =
                                (-(1.0) * enemy.current_velocity_y) + temp_velocity_y;
                        }
                    }
                }
            }
        }
    }
}
