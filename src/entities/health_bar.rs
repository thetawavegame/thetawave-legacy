use amethyst::{
    prelude::Builder,
    ecs::prelude::World,
};

use crate::{
    components::HealthBar,
};


pub fn initialise_health_bar(world: &mut World) {

    world
        .create_entity()
        .with(HealthBar{
            x_pos: 332.0,
            y_pos: 200.0,
            health_stack: vec![],
        })
        .build();
}