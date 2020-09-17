use crate::{
    components::{BlastComponent, BlastType, Spaceship},
    constants::EXPLOSION_Z,
    entities::spawn_blast_explosion,
    resources::SpriteResource,
    space_shooter::HitboxCollisionEvent,
};
use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{Entities, Join, LazyUpdate, ReadExpect, ReadStorage, System, WriteStorage},
    ecs::*,
    ecs::{Read, World},
    shrev::{EventChannel, ReaderId},
};

#[derive(Default)]
pub struct EnemyHitSystem {
    event_reader: Option<ReaderId<HitboxCollisionEvent>>,
}

impl<'s> System<'s> for EnemyHitSystem {
    type SystemData = (
        Read<'s, EventChannel<HitboxCollisionEvent>>,
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
                .fetch_mut::<EventChannel<HitboxCollisionEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (
            collision_channel,
            entities,
            mut spaceships,
            mut blasts,
            transforms,
            sprite_resource,
            lazy_update,
        ): Self::SystemData,
    ) {
        for event in collision_channel.read(self.event_reader.as_mut().unwrap()) {
            for (spaceship_entity, spaceship) in (&entities, &mut spaceships).join() {
                for (blast_entity, blast, blast_transform) in
                    (&*entities, &mut blasts, &transforms).join()
                {
                    //first check if the blast is allied with the player
                    //if the blast collides with the player and the player is not currently barrel rolling the blast hits
                    match blast.blast_type {
                        // using match here for ease of adding enemy blast effects (such as poison) in the future
                        BlastType::Enemy => {
                            if ((event.entity_a == blast_entity
                                && event.entity_b == spaceship_entity)
                                || (event.entity_a == spaceship_entity
                                    && event.entity_b == blast_entity))
                                && !spaceship.barrel_action_left
                                && !spaceship.barrel_action_right
                            {
                                entities
                                    .delete(blast_entity)
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
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
