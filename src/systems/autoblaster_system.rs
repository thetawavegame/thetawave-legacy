use crate::components::AutoBlasterComponent;

use amethyst::{
    core::timing::Time,
    ecs::prelude::{
        Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteStorage,
    },
};

pub struct AutoBlasterSystem;

impl<'s> System<'s> for AutoBlasterSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, AutoBlasterComponent>,
    );

    fn run(&mut self, (transforms, autoblasters): Self::SystemData) {
        for (transform, autoblaster) in (&transforms, &autoblasters) {}
    }
}
