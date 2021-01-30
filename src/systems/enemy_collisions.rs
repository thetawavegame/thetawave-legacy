use crate::{
    audio::Sounds,
    components::{
        BlastComponent, BlastType, CharacterComponent, EnemyComponent, EnemyType, HealthComponent,
        Motion2DComponent,
    },
    entities::spawn_blast_explosion,
    events::{EnemyCollisionEvent, PlayAudioEvent},
    resources::SpriteSheetsResource,
};
use amethyst::{
    core::transform::Transform,
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
        ReadStorage<'s, CharacterComponent>,
        WriteStorage<'s, EnemyComponent>,
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, HealthComponent>,
        Write<'s, EventChannel<PlayAudioEvent>>,
        ReadExpect<'s, Sounds>,
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
        (
            enemy_collision_event_channel,
            characters,
            mut enemies,
            mut motions,
            mut healths,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        for event in enemy_collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            // Is the enemy colliding with an entity with a spaceship component?
            if let Some(character) = characters.get(event.colliding_entity) {
                play_audio_channel.single_write(PlayAudioEvent {
                    source: sounds.sound_effects["metal_crash"].clone(),
                });

                let enemy = enemies.get_mut(event.enemy_entity).unwrap();
                let enemy_motion = motions.get_mut(event.enemy_entity).unwrap();
                let enemy_health = healths.get_mut(event.enemy_entity).unwrap();

                if enemy.name != "repeater_body"
                    && enemy.name != "repeater_head"
                    && enemy.name != "repeater_right_shoulder"
                    && enemy.name != "repeater_left_shoulder"
                    && enemy.name != "repeater_right_arm"
                    && enemy.name != "repeater_left_arm"
                {
                    if let Some(velocity) = event.collision_velocity {
                        enemy_health.value -= character.collision_damage;
                        enemy_motion.velocity.y += velocity.y;
                        enemy_motion.velocity.x += velocity.x;

                        // if velocities are opposite the strafer switches direction
                        if let EnemyType::Strafer = enemy.enemy_type {
                            if enemy_motion.velocity.x * velocity.x < 0.0 {
                                enemy_motion.velocity.x *= -1.0;
                            }
                        }
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
        ReadStorage<'s, EnemyComponent>,
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, HealthComponent>,
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
        (enemy_collision_event_channel, entities, enemies, mut motions, mut healths): Self::SystemData,
    ) {
        for event in enemy_collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            if let Some(colliding_enemy) = enemies.get(event.colliding_entity) {
                for (enemy, enemy_motion, enemy_health, enemy_entity) in
                    (&enemies, &mut motions, &mut healths, &entities).join()
                {
                    if enemy_entity == event.enemy_entity
                        && enemy.name != "repeater_body"
                        && enemy.name != "repeater_head"
                    {
                        if let Some(velocity) = event.collision_velocity {
                            enemy_health.value -= colliding_enemy.collision_damage;
                            enemy_motion.velocity.y += velocity.y;
                            enemy_motion.velocity.x += velocity.x;

                            // if velocities are opposite the strafer switches direction
                            if let EnemyType::Strafer = enemy.enemy_type {
                                if enemy_motion.velocity.x * velocity.x < 0.0 {
                                    enemy_motion.velocity.x *= -1.0;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Default)]
pub struct EnemyBlastCollisionSystem {
    event_reader: Option<ReaderId<EnemyCollisionEvent>>,
}

impl<'s> System<'s> for EnemyBlastCollisionSystem {
    type SystemData = (
        Read<'s, EventChannel<EnemyCollisionEvent>>,
        Entities<'s>,
        WriteStorage<'s, EnemyComponent>,
        WriteStorage<'s, HealthComponent>,
        WriteStorage<'s, BlastComponent>,
        ReadStorage<'s, Transform>,
        ReadExpect<'s, SpriteSheetsResource>,
        ReadExpect<'s, LazyUpdate>,
        Write<'s, EventChannel<PlayAudioEvent>>,
        ReadExpect<'s, Sounds>,
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
        (
            collision_channel,
            entities,
            mut enemies,
            mut healths,
            mut blasts,
            transforms,
            sprite_resource,
            lazy_update,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        for event in collision_channel.read(self.event_reader.as_mut().unwrap()) {
            if let Some(blast) = blasts.get_mut(event.colliding_entity) {
                let enemy = enemies.get_mut(event.enemy_entity).unwrap();
                let enemy_health = healths.get_mut(event.enemy_entity).unwrap();
                let blast_transform = transforms.get(event.colliding_entity).unwrap();

                match blast.blast_type {
                    BlastType::Ally | BlastType::AllyCritical | BlastType::AllyPoison => {
                        entities
                            .delete(event.colliding_entity)
                            .expect("unable to delete entity");

                        play_audio_channel.single_write(PlayAudioEvent {
                            source: sounds.sound_effects["metal_ping"].clone(),
                        });

                        spawn_blast_explosion(
                            &entities,
                            sprite_resource.spritesheets["blast_explosions"].clone(),
                            blast.blast_type.clone(),
                            blast_transform.clone(),
                            &lazy_update,
                        );

                        enemy_health.value -= blast.damage;
                        enemy.poison = blast.poison_damage;
                    }

                    _ => {}
                }
            }
        }
    }
}
