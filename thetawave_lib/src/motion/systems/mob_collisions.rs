use crate::{
    audio::Sounds,
    events::{MobCollisionEvent, PlayAudioEvent},
    misc::components::BarrierComponent,
    misc::resources::GameParametersResource,
    misc::HealthComponent,
    motion::components::Motion2DComponent,
    motion::systems::{barrier_collision, immovable_collision, standard_collision},
    player::PlayerComponent,
    spawnable::{BlastComponent, EffectType, EffectsResource, EnemyType, MobComponent, MobType},
    visual::SpriteSheetsResource,
    weapons::BlastType,
};

use amethyst::{
    core::transform::Transform,
    ecs::*,
    ecs::{Read, System, World},
    shrev::{EventChannel, ReaderId},
};

/// Handles collisions between mobs and players
#[derive(Default)]
pub struct MobPlayerCollisionSystem {
    event_reader: Option<ReaderId<MobCollisionEvent>>,
}

impl<'s> System<'s> for MobPlayerCollisionSystem {
    type SystemData = (
        Read<'s, EventChannel<MobCollisionEvent>>,
        Read<'s, GameParametersResource>,
        ReadStorage<'s, PlayerComponent>,
        WriteStorage<'s, MobComponent>,
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, HealthComponent>,
        Write<'s, EventChannel<PlayAudioEvent>>,
        ReadExpect<'s, Sounds>,
    );

    fn run(
        &mut self,
        (
            mob_collision_event_channel,
            game_parameters,
            players,
            mut mobs,
            mut motions,
            mut healths,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        for event in mob_collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            // Is the mob colliding with an entity with a spaceship component?
            if let Some(player) = players.get(event.colliding_entity) {
                play_audio_channel.single_write(PlayAudioEvent {
                    source: sounds.sound_effects["metal_crash"].clone(),
                });

                let mob = mobs.get_mut(event.mob_entity).unwrap();
                let mob_motion = motions.get_mut(event.mob_entity).unwrap();
                let mob_health = healths.get_mut(event.mob_entity).unwrap();

                match mob.mob_type {
                    MobType::Enemy(EnemyType::Missile) => {
                        mob_health.health.set_health(0.0);
                    }

                    _ => {
                        mob_health.health.take_damage(player.collision_damage);
                    }
                }

                if !mob_motion.immovable {
                    if let Some(collision_velocity) = event.collision_velocity {
                        standard_collision(
                            mob_motion,
                            collision_velocity,
                            game_parameters.min_collision_knockback,
                        );
                    }
                }
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<MobCollisionEvent>>()
                .register_reader(),
        );
    }
}

/// Handles collisions between mobs and other mobs
#[derive(Default)]
pub struct MobMobCollisionSystem {
    event_reader: Option<ReaderId<MobCollisionEvent>>,
}

impl<'s> System<'s> for MobMobCollisionSystem {
    type SystemData = (
        Read<'s, EventChannel<MobCollisionEvent>>,
        Read<'s, GameParametersResource>,
        ReadStorage<'s, MobComponent>,
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, HealthComponent>,
        Write<'s, EventChannel<PlayAudioEvent>>,
        ReadExpect<'s, Sounds>,
    );

    fn run(
        &mut self,
        (
            mob_collision_event_channel,
            game_parameters,
            mobs,
            mut motions,
            mut healths,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        for event in mob_collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            if let Some(colliding_mob) = mobs.get(event.colliding_entity) {
                play_audio_channel.single_write(PlayAudioEvent {
                    source: sounds.sound_effects["metal_crash"].clone(),
                });
                let mob = mobs.get(event.mob_entity).unwrap();
                let mob_motion = motions.get_mut(event.mob_entity).unwrap();
                let mob_health = healths.get_mut(event.mob_entity).unwrap();

                match mob.mob_type {
                    MobType::Enemy(EnemyType::Missile) => {
                        mob_health.health.set_health(0.0);
                    }

                    _ => {
                        mob_health
                            .health
                            .take_damage(colliding_mob.collision_damage);
                    }
                }

                if !mob_motion.immovable {
                    if let Some(collision_velocity) = event.collision_velocity {
                        if event.collider_immovable {
                            immovable_collision(
                                mob_motion,
                                collision_velocity,
                                game_parameters.min_collision_knockback,
                            );
                        } else {
                            standard_collision(
                                mob_motion,
                                collision_velocity,
                                game_parameters.min_collision_knockback,
                            );
                        }
                    }
                }
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<MobCollisionEvent>>()
                .register_reader(),
        );
    }
}

/// Handles collisions between mobs and blasts
#[derive(Default)]
pub struct MobBlastCollisionSystem {
    event_reader: Option<ReaderId<MobCollisionEvent>>,
}

impl<'s> System<'s> for MobBlastCollisionSystem {
    type SystemData = (
        Read<'s, EventChannel<MobCollisionEvent>>,
        Entities<'s>,
        WriteStorage<'s, HealthComponent>,
        WriteStorage<'s, BlastComponent>,
        ReadStorage<'s, Transform>,
        ReadExpect<'s, EffectsResource>,
        ReadExpect<'s, SpriteSheetsResource>,
        ReadExpect<'s, LazyUpdate>,
        Write<'s, EventChannel<PlayAudioEvent>>,
        ReadExpect<'s, Sounds>,
    );

    fn run(
        &mut self,
        (
            mob_collision_event_channel,
            entities,
            mut healths,
            mut blasts,
            transforms,
            effects_resource,
            sprite_resource,
            lazy_update,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        for event in mob_collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            if let Some(blast) = blasts.get_mut(event.colliding_entity) {
                let mob_health = healths.get_mut(event.mob_entity).unwrap();
                let blast_transform = transforms.get(event.colliding_entity).unwrap();

                match blast.blast_type {
                    BlastType::Ally | BlastType::AllyCritical | BlastType::AllyPoison => {
                        entities
                            .delete(event.colliding_entity)
                            .expect("unable to delete entity");

                        play_audio_channel.single_write(PlayAudioEvent {
                            source: sounds.sound_effects["metal_ping"].clone(),
                        });

                        effects_resource.spawn_effect(
                            match blast.blast_type {
                                BlastType::Ally => &EffectType::AllyBlastExplosion,
                                BlastType::AllyCritical => &EffectType::CriticalBlastExplosion,
                                BlastType::AllyPoison => &EffectType::PoisonBlastExplosion,
                                _ => panic!("unreachable"),
                            },
                            blast_transform,
                            &sprite_resource,
                            &entities,
                            &lazy_update,
                        );

                        mob_health.health.take_damage(blast.damage);

                        //TODO: apply poison to enemy health component from blast
                        //enemy.poison = blast.poison_damage;
                    }

                    _ => {}
                }
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<MobCollisionEvent>>()
                .register_reader(),
        );
    }
}

/// Handles collision between mobs and arena borders
#[derive(Default)]
pub struct MobArenaBorderCollisionSystem {
    event_reader: Option<ReaderId<MobCollisionEvent>>,
}

impl<'s> System<'s> for MobArenaBorderCollisionSystem {
    type SystemData = (
        Read<'s, EventChannel<MobCollisionEvent>>,
        ReadStorage<'s, BarrierComponent>,
        ReadStorage<'s, MobComponent>,
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, HealthComponent>,
        Write<'s, EventChannel<PlayAudioEvent>>,
        ReadExpect<'s, Sounds>,
    );

    fn run(
        &mut self,
        (
            mob_collision_event_channel,
            barriers,
            mobs,
            mut motion_2ds,
            mut healths,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        for event in mob_collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            // is the mob colliding with a barrier?
            if let Some(barrier) = barriers.get(event.colliding_entity) {
                let mob = mobs.get(event.mob_entity).unwrap();

                if !barrier.enemies_pass {
                    match mob.mob_type {
                        MobType::Enemy(EnemyType::Missile) => {}

                        _ => {
                            let mob_motion = motion_2ds.get_mut(event.mob_entity).unwrap();
                            let mob_health = healths.get_mut(event.mob_entity).unwrap();

                            barrier_collision(mob_motion, barrier);

                            mob_health.health.take_damage(barrier.damage);

                            play_audio_channel.single_write(PlayAudioEvent {
                                source: sounds.sound_effects["force_field"].clone(),
                            });
                        }
                    }
                }
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<MobCollisionEvent>>()
                .register_reader(),
        );
    }
}
