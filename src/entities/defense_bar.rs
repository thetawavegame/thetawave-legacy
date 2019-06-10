use amethyst::{
    prelude::Builder,
    ecs::prelude::World,
};

use crate::{
    components::DefenseBar,
};


const DEFENSE: f32 = 500.0;
const X_POSITION: f32 = 352.0;
const Y_POSITION: f32 = 200.0;


pub fn initialise_defense_bar(world: &mut World) {

    world
        .create_entity()
        .with(DefenseBar{
            x_pos: X_POSITION,
            y_pos: Y_POSITION,
            defense_stack: vec![],
            defense: DEFENSE,
            max_defense: DEFENSE,
        })
        .build();
}