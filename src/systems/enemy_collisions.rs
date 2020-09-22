use crate::{
    audio::play_sfx,
    components::{Enemy, Motion2DComponent, Spaceship},
    constants::SPACESHIP_COLLISION_DAMAGE,
    space_shooter::EnemyCollisionEvent,
};
use amethyst::{
    ecs::*,
    ecs::{Read, System, World},
    shrev::{EventChannel, ReaderId},
};

#[derive(Default)]
pub struct EnemyPlayerCollisionSystem {
    event_reader: Option<ReaderId<EnemyCollisionEvent>>,
}

impl<'s> System<'s> for EnemyPlayerCollisionSystem {
    type SystemData = (
        Read<'s, EventChannel<EnemyCollisionEvent>>,
        ReadStorage<'s, Spaceship>,
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Motion2DComponent>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<EnemyCollisionEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (enemy_collision_event_channel, spaceships, mut enemies, mut motions): Self::SystemData,
    ) {
        for event in enemy_collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            // play_sfx(&sounds.crash_sfx, &storage, audio_output.as_deref());

            // Is the player colliding with an entity with an enemy component?
            if let Some(spaceship) = spaceships.get(event.colliding_entity) {
                let enemy = enemies.get_mut(event.enemy_entity).unwrap();
                let enemy_motion = motions.get_mut(event.enemy_entity).unwrap();

                if enemy.name != "repeater_body"
                    && enemy.name != "repeater_head"
                    && enemy.name != "repeater_right_shoulder"
                    && enemy.name != "repeater_left_shoulder"
                    && enemy.name != "repeater_right_arm"
                    && enemy.name != "repeater_left_arm"
                {
                    println!("enemy spaceship collision");
                    if let Some(velocity) = event.collision_velocity {
                        println!("collision velocity enemy");
                        enemy.health -= spaceship.collision_damage;
                        enemy_motion.velocity.x += velocity.x;
                        enemy_motion.velocity.y += velocity.y;
                    }
                }
            }
        }
    }
}

#[derive(Default)]
pub struct EnemyEnemyCollisionSystem {
    event_reader: Option<ReaderId<EnemyCollisionEvent>>,
}

impl<'s> System<'s> for EnemyEnemyCollisionSystem {
    type SystemData = (
        Read<'s, EventChannel<EnemyCollisionEvent>>,
        Entities<'s>,
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Motion2DComponent>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<EnemyCollisionEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (enemy_collision_event_channel, entities, mut enemies, mut motions): Self::SystemData,
    ) {
        for event in enemy_collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            if let Some(_colliding_enemy) = enemies.get(event.colliding_entity) {
                for (enemy, enemy_motion, enemy_entity) in
                    (&mut enemies, &mut motions, &entities).join()
                {
                    if enemy_entity == event.enemy_entity
                        && enemy.name != "repeater_body"
                        && enemy.name != "repeater_head"
                    {
                        if let Some(velocity) = event.collision_velocity {
                            enemy.health -= SPACESHIP_COLLISION_DAMAGE;
                            enemy_motion.velocity.x = velocity.x;
                            enemy_motion.velocity.y = velocity.y;
                        }
                    } else if enemy_entity == event.colliding_entity {
                        if let Some(velocity) = event.collision_velocity {
                            enemy.health -= SPACESHIP_COLLISION_DAMAGE;
                            enemy_motion.velocity.x = velocity.x;
                            enemy_motion.velocity.y = velocity.y;
                        }
                    }
                }
            }
        }
    }
}
