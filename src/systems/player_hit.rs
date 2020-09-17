use crate::{
    audio::{play_sfx, Sounds},
    components::{BlastComponent, BlastType, Enemy, Spaceship},
    constants::EXPLOSION_Z,
    entities::spawn_blast_explosion,
    resources::SpriteResource,
    space_shooter::HitboxCollisionEvent,
};
use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{
        Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteStorage,
    },
    ecs::World,
    ecs::*,
    shrev::{EventChannel, ReaderId},
};

#[derive(Default)]
pub struct PlayerHitSystem {
    event_reader: Option<ReaderId<HitboxCollisionEvent>>,
}

impl<'s> System<'s> for PlayerHitSystem {
    type SystemData = (
        Read<'s, EventChannel<HitboxCollisionEvent>>,
        Entities<'s>,
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, BlastComponent>,
        ReadStorage<'s, Transform>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<HitboxCollisionEvent>>()
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
            storage,
            sounds,
            audio_output,
            sprite_resource,
            lazy_update,
        ): Self::SystemData,
    ) {
        for event in collision_channel.read(self.event_reader.as_mut().unwrap()) {
            for (enemy_entity, enemy) in (&entities, &mut enemies).join() {
                for (blast_entity, blast, blast_transform) in
                    (&entities, &mut blasts, &transforms).join()
                {
                    match blast.blast_type {
                        BlastType::Ally | BlastType::AllyCritical | BlastType::AllyPoison => {
                            if (event.entity_a == blast_entity && event.entity_b == enemy_entity)
                                || (event.entity_a == enemy_entity
                                    && event.entity_b == blast_entity)
                            {
                                entities
                                    .delete(blast_entity)
                                    .expect("unable to delete entity");
                                play_sfx(
                                    &sounds.spaceship_hit_sfx,
                                    &storage,
                                    audio_output.as_deref(),
                                );

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
                        }

                        _ => {}
                    }
                }
            }
        }
    }
}
