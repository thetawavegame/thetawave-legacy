use amethyst::{
    prelude::Builder,
    ecs::prelude::World,
};

use crate::{
    components::DefenseBar,
};

const DEFENSE: f32 = 500.0;


pub fn initialise_defense_bar(world: &mut World) {

    world
        .create_entity()
        .with(DefenseBar{
            x_pos: 352.0,
            y_pos: 200.0,
            defense_stack: vec![],
            defense: DEFENSE,
            max_defense: DEFENSE,
        })
        .build();
}