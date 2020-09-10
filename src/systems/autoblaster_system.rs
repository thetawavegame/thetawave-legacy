use crate::{
    components::{AutoBlasterComponent, Motion2DComponent},
    resources::SpriteResource,
};

use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{
        Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteStorage,
    },
};

pub struct AutoBlasterSystem;

impl<'s> System<'s> for AutoBlasterSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, Time>,
        ReadExpect<'s, LazyUpdate>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, AutoBlasterComponent>,
        ReadStorage<'s, Motion2DComponent>,
        ReadExpect<'s, SpriteResource>,
    );

    fn run(
        &mut self,
        (entities, time, lazy_update, transforms, mut autoblasters, motion2ds, sprite_resource): Self::SystemData,
    ) {
        for (transform, autoblaster, motion2d) in
            (&transforms, &mut autoblasters, &motion2ds).join()
        {
            autoblaster.fire_when_ready(
                motion2d,
                transform,
                time.delta_seconds(),
                &entities,
                &sprite_resource,
                &lazy_update,
            );
        }
    }
}
