use crate::{
    audio::Sounds,
    components::{
        BarrierComponent, BlastComponent, BlastType, ConsumableComponent, DefenseTag,
        EnemyComponent, HealthComponent, ItemComponent, Motion2DComponent, PlayerComponent,
    },
    entities::spawn_blast_explosion,
    events::{ItemGetEvent, PlayAudioEvent, PlayerCollisionEvent},
    resources::SpriteSheetsResource,
    systems::standard_collision,
};
use amethyst::{
    core::transform::Transform,
    ecs::*,
    shrev::{EventChannel, ReaderId},
};

#[derive(Default)]
pub struct SpaceshipEnemyCollisionSystem {
    event_reader: Option<ReaderId<PlayerCollisionEvent>>,
}

impl<'s> System<'s> for SpaceshipEnemyCollisionSystem {
    type SystemData = (
        Read<'s, EventChannel<PlayerCollisionEvent>>,
        ReadStorage<'s, EnemyComponent>,
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, HealthComponent>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<PlayerCollisionEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (collision_event_channel, enemies, mut motions, mut healths): Self::SystemData,
    ) {
        for event in collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            // Is the player colliding with an enemy entity?
            if let Some(enemy) = enemies.get(event.colliding_entity) {
                let spaceship_motion = motions.get_mut(event.player_entity).unwrap();
                let spaceship_health = healths.get_mut(event.player_entity).unwrap();

                spaceship_health.take_damage(enemy.collision_damage);

                if let Some(collision_velocity) = event.collision_velocity {
                    standard_collision(spaceship_motion, collision_velocity, 50.0);
                }
            }
        }
    }
}

#[derive(Default)]
pub struct SpaceshipBlastCollisionSystem {
    event_reader: Option<ReaderId<PlayerCollisionEvent>>,
}

impl<'s> System<'s> for SpaceshipBlastCollisionSystem {
    type SystemData = (
        Read<'s, EventChannel<PlayerCollisionEvent>>,
        Entities<'s>,
        WriteStorage<'s, HealthComponent>,
        WriteStorage<'s, BlastComponent>,
        ReadStorage<'s, Transform>,
        ReadExpect<'s, SpriteSheetsResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<PlayerCollisionEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (
            collision_event_channel,
            entities,
            mut healths,
            mut blasts,
            transforms,
            sprite_resource,
            lazy_update,
        ): Self::SystemData,
    ) {
        for event in collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            // Is the player colliding with an entity with a blast component?
            if let Some(blast) = blasts.get_mut(event.colliding_entity) {
                let spaceship_health = healths.get_mut(event.player_entity).unwrap();
                let blast_transform = transforms.get(event.colliding_entity).unwrap();

                // first check if the blast is allied with the player
                // TODO blast should not hit if player is currently barrel rolling
                match blast.blast_type {
                    // using match here for ease of adding enemy blast effects (such as poison) in the future
                    BlastType::Enemy => {
                        entities
                            .delete(event.colliding_entity)
                            .expect("unable to delete entity");

                        spawn_blast_explosion(
                            &entities,
                            sprite_resource.spritesheets["blast_explosions"].clone(),
                            blast.blast_type.clone(),
                            blast_transform.clone(),
                            &lazy_update,
                        );
                        spaceship_health.take_damage(blast.damage);
                    }
                    _ => {}
                }
            }
        }
    }
}

#[derive(Default)]
pub struct SpaceshipItemCollisionSystem {
    event_reader: Option<ReaderId<PlayerCollisionEvent>>,
}

impl<'s> System<'s> for SpaceshipItemCollisionSystem {
    type SystemData = (
        Read<'s, EventChannel<PlayerCollisionEvent>>,
        Entities<'s>,
        ReadStorage<'s, ItemComponent>,
        Write<'s, EventChannel<ItemGetEvent>>,
        Write<'s, EventChannel<PlayAudioEvent>>,
        ReadExpect<'s, Sounds>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<PlayerCollisionEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (
            collision_event_channel,
            entities,
            items,
            mut item_get_event_channel,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        for event in collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            // Is the player colliding with an entity with an item component?
            if let Some(item) = items.get(event.colliding_entity) {
                item_get_event_channel.single_write(ItemGetEvent::new(
                    event.player_entity,
                    item.stat_effects.clone(),
                    item.bool_effects.clone(),
                ));

                play_audio_channel.single_write(PlayAudioEvent {
                    source: sounds.sound_effects["shotgun_cock"].clone(),
                });

                entities
                    .delete(event.colliding_entity)
                    .expect("unable to delete entity");
            }
        }
    }
}

#[derive(Default)]
pub struct SpaceshipConsumableCollisionSystem {
    event_reader: Option<ReaderId<PlayerCollisionEvent>>,
}

impl<'s> System<'s> for SpaceshipConsumableCollisionSystem {
    type SystemData = (
        Read<'s, EventChannel<PlayerCollisionEvent>>,
        Entities<'s>,
        ReadStorage<'s, ConsumableComponent>,
        WriteStorage<'s, PlayerComponent>,
        ReadStorage<'s, DefenseTag>,
        WriteStorage<'s, HealthComponent>,
        Write<'s, EventChannel<PlayAudioEvent>>,
        ReadExpect<'s, Sounds>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<PlayerCollisionEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (
            collision_event_channel,
            entities,
            consumables,
            mut players,
            defense_tags,
            mut healths,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        for event in collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            // Is the player colliding with an entity with a consumable entity?
            if let Some(consumable) = consumables.get(event.colliding_entity) {
                let spaceship_health = healths.get_mut(event.player_entity).unwrap();
                let player = players.get_mut(event.player_entity).unwrap();

                spaceship_health.value += consumable.health_value;
                spaceship_health.armor += consumable.armor_value;
                player.money += consumable.money_value;
                for (_defense_tag, defense_health) in (&defense_tags, &mut healths).join() {
                    defense_health.value += consumable.defense_value;
                }

                play_audio_channel.single_write(PlayAudioEvent {
                    source: sounds.sound_effects[&consumable.sound_effect].clone(),
                });

                entities
                    .delete(event.colliding_entity)
                    .expect("unable to delete entity");
            }
        }
    }
}

#[derive(Default)]
pub struct SpaceshipArenaBorderCollisionSystem {
    event_reader: Option<ReaderId<PlayerCollisionEvent>>,
}

impl<'s> System<'s> for SpaceshipArenaBorderCollisionSystem {
    type SystemData = (
        Read<'s, EventChannel<PlayerCollisionEvent>>,
        ReadStorage<'s, BarrierComponent>,
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, HealthComponent>,
        Write<'s, EventChannel<PlayAudioEvent>>,
        ReadExpect<'s, Sounds>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<PlayerCollisionEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (
            collision_event_channel,
            barriers,
            mut motion_2ds,
            mut healths,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        for event in collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            // Is the player colliding with a barrier?
            if let Some(barrier) = barriers.get(event.colliding_entity) {
                if let Some(collision_velocity) = event.collision_velocity {
                    let player_motion_2d = motion_2ds.get_mut(event.player_entity).unwrap();
                    let player_health = healths.get_mut(event.player_entity).unwrap();

                    // set velocity to deflection velocity if collision velocity under deflection velocity
                    if barrier.deflection_velocity.x.abs() > 0.0 {
                        if collision_velocity.x.abs() < barrier.deflection_velocity.x.abs() {
                            player_motion_2d.velocity.x = barrier.deflection_velocity.x;
                        } else {
                            player_motion_2d.velocity.x = collision_velocity.x;
                        }
                    }

                    if barrier.deflection_velocity.y.abs() > 0.0 {
                        if collision_velocity.y.abs() < barrier.deflection_velocity.y.abs() {
                            player_motion_2d.velocity.y = barrier.deflection_velocity.y;
                        } else {
                            player_motion_2d.velocity.y = collision_velocity.y;
                        }
                    }

                    player_health.value -= barrier.damage;
                }
                play_audio_channel.single_write(PlayAudioEvent {
                    source: sounds.sound_effects["force_field"].clone(),
                });
            }
        }
    }
}
