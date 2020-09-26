use crate::{
    components::{DefenseTag, HealthComponent},
    constants::DEFENSE,
};
use amethyst::{
    ecs::{World, WorldExt},
    prelude::Builder,
};

pub fn initialize_defense(world: &mut World) {
    world
        .create_entity()
        .with(DefenseTag::default())
        .with(HealthComponent {
            health: DEFENSE,
            max_health: DEFENSE,
        })
        .build();
}
