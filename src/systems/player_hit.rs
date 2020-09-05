use crate::{
    audio::{play_sfx, Sounds},
    components::{Blast, Enemy},
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
        WriteStorage<'s, Blast>,
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
                    if blast.allied
                        && ((event.entity_a == blast_entity && event.entity_b == enemy_entity)
                            || (event.entity_a == enemy_entity && event.entity_b == blast_entity))
                    {
                        let _result = entities.delete(blast_entity);
                        play_sfx(&sounds.spaceship_hit_sfx, &storage, audio_output.as_deref());

                        let explosion_position = Vector3::new(
                            blast_transform.translation().x,
                            blast_transform.translation().y,
                            EXPLOSION_Z,
                        );

                        spawn_blast_explosion(
                            &entities,
                            sprite_resource.blast_explosions_sprite_sheet.clone(),
                            blast.blast_type.clone(),
                            explosion_position,
                            &lazy_update,
                        );

                        enemy.health -= blast.damage;
                        enemy.poison = blast.poison_damage;
                    }
                }
            }
        }
    }
}
