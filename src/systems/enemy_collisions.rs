use crate::{
    audio::{play_sfx, Sounds},
    components::{BlastComponent, BlastType, Enemy, Motion2DComponent, Spaceship},
    constants::SPACESHIP_COLLISION_DAMAGE,
    entities::spawn_blast_explosion,
    resources::SpriteResource,
    space_shooter::EnemyCollisionEvent,
};
use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
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
        ReadStorage<'s, Spaceship>,
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Motion2DComponent>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
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
            spaceships,
            mut enemies,
            mut motions,
            storage,
            sounds,
            audio_output,
        ): Self::SystemData,
    ) {
        for event in enemy_collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            // Is the enemy colliding with an entity with a spaceship component?
            if let Some(spaceship) = spaceships.get(event.colliding_entity) {
                play_sfx(&sounds.crash_sfx, &storage, audio_output.as_deref());

                let enemy = enemies.get_mut(event.enemy_entity).unwrap();
                let enemy_motion = motions.get_mut(event.enemy_entity).unwrap();

                if enemy.name != "repeater_body"
                    && enemy.name != "repeater_head"
                    && enemy.name != "repeater_right_shoulder"
                    && enemy.name != "repeater_left_shoulder"
                    && enemy.name != "repeater_right_arm"
                    && enemy.name != "repeater_left_arm"
                {
                    if let Some(velocity) = event.collision_velocity {
                        enemy.health -= spaceship.collision_damage;
                        enemy_motion.velocity.x = -enemy_motion.velocity.x + velocity.x;
                        enemy_motion.velocity.y = -enemy_motion.velocity.y + velocity.y;
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
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, BlastComponent>,
        ReadStorage<'s, Transform>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
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
            mut blasts,
            transforms,
            sprite_resource,
            lazy_update,
            storage,
            sounds,
            audio_output,
        ): Self::SystemData,
    ) {
        for event in collision_channel.read(self.event_reader.as_mut().unwrap()) {
            if let Some(blast) = blasts.get_mut(event.colliding_entity) {
                let enemy = enemies.get_mut(event.enemy_entity).unwrap();
                let blast_transform = transforms.get(event.colliding_entity).unwrap();

                match blast.blast_type {
                    BlastType::Ally | BlastType::AllyCritical | BlastType::AllyPoison => {
                        entities
                            .delete(event.colliding_entity)
                            .expect("unable to delete entity");

                        play_sfx(&sounds.spaceship_hit_sfx, &storage, audio_output.as_deref());

                        spawn_blast_explosion(
                            &entities,
                            sprite_resource.blast_explosions_sprite_sheet.clone(),
                            blast.blast_type.clone(),
                            blast_transform.clone(),
                            &lazy_update,
                        );

                        enemy.health -= blast.damage;
                        enemy.poison = blast.poison_damage;
                    }

                    _ => {}
                }
            }
        }
    }
}
