use crate::{
    audio::Sounds,
    components::{
        BlastComponent, BlastType, ConsumableComponent, DefenseTag, EnemyComponent,
        HealthComponent, ItemComponent, Motion2DComponent, SpaceshipComponent,
    },
    entities::spawn_blast_explosion,
    events::{ItemGetEvent, PlayAudioEvent, PlayerCollisionEvent},
    resources::SpriteResource,
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
        WriteStorage<'s, SpaceshipComponent>,
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
        (collision_event_channel, enemies, mut spaceships, mut motions, mut healths): Self::SystemData,
    ) {
        for event in collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            // Is the player colliding with an entity with an enemy component?
            if let Some(enemy) = enemies.get(event.colliding_entity) {
                let spaceship = spaceships.get_mut(event.player_entity).unwrap();
                let spaceship_motion = motions.get_mut(event.player_entity).unwrap();
                let spaceship_health = healths.get_mut(event.player_entity).unwrap();

                if spaceship.barrel_action_left {
                    spaceship.barrel_action_right = true;
                    spaceship.barrel_action_left = false;
                } else if spaceship.barrel_action_right {
                    spaceship.barrel_action_left = true;
                    spaceship.barrel_action_right = false;
                }

                if !spaceship.steel_barrel
                    || (!spaceship.barrel_action_left && !spaceship.barrel_action_right)
                {
                    spaceship_health.value -= enemy.collision_damage;
                }

                if let Some(velocity) = event.collision_velocity {
                    // Push the ship in the opposite direction.
                    spaceship_motion.velocity.x = velocity.x - spaceship_motion.velocity.x;
                    spaceship_motion.velocity.y = velocity.y - spaceship_motion.velocity.y;
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
        ReadExpect<'s, SpriteResource>,
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
                            sprite_resource.blast_explosions_sprite_sheet.clone(),
                            blast.blast_type.clone(),
                            blast_transform.clone(),
                            &lazy_update,
                        );
                        spaceship_health.value -= blast.damage;
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
                    source: sounds.item_sfx.clone(),
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
        WriteStorage<'s, SpaceshipComponent>,
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
            mut spaceships,
            defense_tags,
            mut healths,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        for event in collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            // Is the player colliding with an entity with an item component?
            if let Some(consumable) = consumables.get(event.colliding_entity) {
                let spaceship = spaceships.get_mut(event.player_entity).unwrap();
                let spaceship_health = healths.get_mut(event.player_entity).unwrap();
                spaceship_health.value += consumable.health_value;
                spaceship.money += consumable.money_value;

                if consumable.money_value == 1 {
                    play_audio_channel.single_write(PlayAudioEvent {
                        source: sounds.small_rock_sfx.clone(),
                    });
                } else if consumable.money_value == 5 {
                    play_audio_channel.single_write(PlayAudioEvent {
                        source: sounds.large_rock_sfx.clone(),
                    });
                } else if consumable.health_value > 0.0 || consumable.defense_value > 0.0 {
                    play_audio_channel.single_write(PlayAudioEvent {
                        source: sounds.wrench_sfx.clone(),
                    });
                }

                for (_defense_tag, defense_health) in (&defense_tags, &mut healths).join() {
                    defense_health.value += consumable.defense_value;
                }

                entities
                    .delete(event.colliding_entity)
                    .expect("unable to delete entity");
            }
        }
    }
}
