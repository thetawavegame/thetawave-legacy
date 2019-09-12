use amethyst::{
    prelude::Builder,
    ecs::prelude::World,
};
use crate::{
    components::Defense,
};

const DEFENSE: f32 = 500.0;

pub fn initialise_defense(world: &mut World) {
    world
        .create_entity()
        .with(Defense{
            defense: DEFENSE,
            max_defense: DEFENSE,
        })
        .build();
}