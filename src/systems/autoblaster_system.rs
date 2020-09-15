use crate::{
    components::{
        AutoFireComponent, BlastComponent, BlastType, BlasterComponent, Hitbox2DComponent,
        Motion2DComponent,
    },
    constants::{
        BLAST_HITBOX_DIAMETER, BLAST_Z, CRIT_BLAST_SPRITE_INDEX, ENEMY_BLAST_SPRITE_INDEX,
        PLAYER_BLAST_SPRITE_INDEX, POISON_BLAST_SPRITE_INDEX,
    },
    entities::spawn_blasts,
    resources::SpriteResource,
};

use amethyst::{
    core::{
        math::{Vector2, Vector3},
        timing::Time,
        transform::Transform,
    },
    ecs::prelude::{
        Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteStorage,
    },
    renderer::SpriteRender,
};

use rand::{thread_rng, Rng};

pub struct AutoBlasterSystem;

impl<'s> System<'s> for AutoBlasterSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, Time>,
        ReadExpect<'s, LazyUpdate>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, BlasterComponent>,
        WriteStorage<'s, AutoFireComponent>,
        ReadStorage<'s, Motion2DComponent>,
        ReadExpect<'s, SpriteResource>,
    );

    fn run(
        &mut self,
        (
            entities,
            time,
            lazy_update,
            transforms,
            blasters,
            mut autofires,
            motion2ds,
            sprite_resource,
        ): Self::SystemData,
    ) {
        for (transform, autofire, blaster, motion2d) in
            (&transforms, &mut autofires, &blasters, &motion2ds).join()
        {
            if autofire.fire_timer > 0.0 {
                autofire.fire_timer -= time.delta_seconds();
            } else {
                autofire.fire_timer = autofire.fire_period;
                blaster.fire(
                    motion2d,
                    transform,
                    &entities,
                    &sprite_resource,
                    &lazy_update,
                );
            }
        }
    }
}
