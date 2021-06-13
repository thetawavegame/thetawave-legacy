use crate::{
    motion::components::Motion2DComponent,
    resources::SpriteSheetsResource,
    weapons::components::{AutoFireComponent, BlasterComponent},
};
use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{
        Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteStorage,
    },
};

/// Handles periodic firing of weapons
pub struct AutoFireSystem;

impl<'s> System<'s> for AutoFireSystem {
    /// Data used by the system
    type SystemData = (
        Entities<'s>,
        Read<'s, Time>,
        ReadExpect<'s, LazyUpdate>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, BlasterComponent>,
        WriteStorage<'s, AutoFireComponent>,
        ReadStorage<'s, Motion2DComponent>,
        ReadExpect<'s, SpriteSheetsResource>,
    );

    /// System game logic
    fn run(
        &mut self,
        (
            entities,
            time,
            lazy_update,
            transforms,
            blasters,
            mut auto_fires,
            motion2ds,
            sprite_resource,
        ): Self::SystemData,
    ) {
        for (transform, auto_fire, blaster, motion2d) in
            (&transforms, &mut auto_fires, &blasters, &motion2ds).join()
        {
            if auto_fire.timer > 0.0 {
                auto_fire.timer -= time.delta_seconds();
            } else {
                auto_fire.timer = auto_fire.period;
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
