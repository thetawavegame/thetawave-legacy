use crate::{components::Defense, constants::DEFENSE};
use amethyst::{
    ecs::{World, WorldExt},
    prelude::Builder,
};

pub fn initialise_defense(world: &mut World) {
    world
        .create_entity()
        .with(Defense {
            defense: DEFENSE,
            max_defense: DEFENSE,
        })
        .build();
}
