use amethyst::{
    prelude::Builder,
    ecs::prelude::World,
};

use crate::{
    components::RollBar,
};


const X_POSITION: f32 = 324.0;
const Y_POSITION: f32 = 177.0;


pub fn initialise_roll_bar(world: &mut World) {

    world
        .create_entity()
        .with(RollBar{
            x_pos: X_POSITION,
            y_pos: Y_POSITION,
            roll_stack: vec![],
        })
        .build();
}