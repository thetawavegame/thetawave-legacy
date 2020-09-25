use crate::{
    audio::{play_sfx, Sounds},
    components::{
        BlastComponent, BlastType, BlasterComponent, Consumable, Defense, Enemy, Item, Living,
        ManualFireComponent, Motion2DComponent, Spaceship,
    },
    entities::spawn_blast_explosion,
    events::PlayerCollisionEvent,
    resources::SpriteResource,
};
use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
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
        ReadStorage<'s, Enemy>,
        WriteStorage<'s, Spaceship>,
        WriteStorage<'s, Motion2DComponent>,
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
        (collision_event_channel, enemies, mut spaceships, mut motions): Self::SystemData,
    ) {
        for event in collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            // Is the player colliding with an entity with an enemy component?
            if let Some(enemy) = enemies.get(event.colliding_entity) {
                let spaceship = spaceships.get_mut(event.player_entity).unwrap();
                let spaceship_motion = motions.get_mut(event.player_entity).unwrap();

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
        WriteStorage<'s, Spaceship>,
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
            mut spaceships,
            mut blasts,
            transforms,
            sprite_resource,
            lazy_update,
        ): Self::SystemData,
    ) {
        for event in collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            // Is the player colliding with an entity with a blast component?
            if let Some(blast) = blasts.get_mut(event.colliding_entity) {
                let spaceship = spaceships.get_mut(event.player_entity).unwrap();
                let blast_transform = transforms.get(event.colliding_entity).unwrap();

                //first check if the blast is allied with the player
                //if the blast collides with the player and the player is not currently barrel rolling the blast hits
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
                        spaceship.health -= blast.damage;
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
        ReadStorage<'s, Item>,
        WriteStorage<'s, Spaceship>,
        WriteStorage<'s, Defense>,
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, BlasterComponent>,
        WriteStorage<'s, ManualFireComponent>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
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
            mut spaceships,
            mut defenses,
            mut motions,
            mut blasters,
            mut manual_fires,
            storage,
            sounds,
            audio_output,
        ): Self::SystemData,
    ) {
        for event in collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            // Is the player colliding with an entity with an item component?
            if let Some(item) = items.get(event.colliding_entity) {
                let spaceship = spaceships.get_mut(event.player_entity).unwrap();
                let blaster = blasters.get_mut(event.player_entity).unwrap();
                let manual_fire = manual_fires.get_mut(event.player_entity).unwrap();
                let motion = motions.get_mut(event.player_entity).unwrap();

                if item.stat_effects.contains_key("max_defense") {
                    for defense in (&mut defenses).join() {
                        // increases maximum capacity for defense
                        defense.set_max_health(
                            defense.max_health() + item.stat_effects["max_defense"],
                        );
                        // sets current defense to new maximum value
                        defense.set_health(defense.health() + item.stat_effects["max_defense"]);
                    }
                }

                // add stats to spaceship
                if item.bool_effects.contains_key("barrel_immunity") {
                    spaceship.steel_barrel = item.bool_effects["barrel_immunity"];
                }

                if item.stat_effects.contains_key("blast_count") {
                    blaster.count += item.stat_effects["blast_count"] as usize;
                }

                if item.stat_effects.contains_key("fire_speed") {
                    manual_fire.period += item.stat_effects["fire_speed"];
                }

                if item.stat_effects.contains_key("damage") {
                    blaster.damage += item.stat_effects["damage"];
                }

                if item.stat_effects.contains_key("max_speed") {
                    motion.max_speed.x += item.stat_effects["max_speed"];
                    motion.max_speed.y += item.stat_effects["max_speed"];
                }
                if item.stat_effects.contains_key("crit_chance") {
                    blaster.crit_chance += item.stat_effects["crit_chance"];
                }

                if item.stat_effects.contains_key("poison_chance") {
                    blaster.poison_chance += item.stat_effects["poison_chance"];
                }

                if item.stat_effects.contains_key("barrel_cooldown") {
                    spaceship.barrel_cooldown += item.stat_effects["barrel_cooldown"];
                }

                if item.stat_effects.contains_key("acceleration") {
                    motion.acceleration.x += item.stat_effects["acceleration"];
                    motion.acceleration.y += item.stat_effects["acceleration"];
                }

                if item.stat_effects.contains_key("deceleration") {
                    motion.deceleration.x += item.stat_effects["deceleration"];
                    motion.deceleration.y += item.stat_effects["deceleration"];
                }

                if item.stat_effects.contains_key("health_multiply") {
                    spaceship.set_max_health(
                        spaceship.max_health()
                            + (item.stat_effects["health_multiply"] * spaceship.max_health),
                    );
                    spaceship.set_health(spaceship.max_health)
                }

                if item.stat_effects.contains_key("health_add") {
                    spaceship
                        .set_max_health(spaceship.max_health() + item.stat_effects["health_add"]);
                    spaceship.set_health(spaceship.max_health);
                }

                if item.stat_effects.contains_key("blast_size_multiplier") {
                    blaster.size_multiplier += item.stat_effects["blast_size_multiplier"];
                }

                play_sfx(&sounds.item_sfx, &storage, audio_output.as_deref());

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
        ReadStorage<'s, Consumable>,
        WriteStorage<'s, Spaceship>,
        WriteStorage<'s, Defense>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
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
            mut defenses,
            storage,
            sounds,
            audio_output,
        ): Self::SystemData,
    ) {
        for event in collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            // Is the player colliding with an entity with an item component?
            if let Some(consumable) = consumables.get(event.colliding_entity) {
                let spaceship = spaceships.get_mut(event.player_entity).unwrap();
                spaceship.health += consumable.health_value;
                spaceship.money += consumable.money_value;

                if consumable.money_value == 1 {
                    play_sfx(&sounds.small_rock_sfx, &storage, audio_output.as_deref());
                } else if consumable.money_value == 5 {
                    play_sfx(&sounds.large_rock_sfx, &storage, audio_output.as_deref());
                } else if consumable.health_value > 0.0 || consumable.defense_value > 0.0 {
                    play_sfx(&sounds.wrench_sfx, &storage, audio_output.as_deref());
                }

                for defense in (&mut defenses).join() {
                    defense.defense += consumable.defense_value;
                }

                entities
                    .delete(event.colliding_entity)
                    .expect("unable to delete entity");
            }
        }
    }
}
