use crate::{
    components::{StatusBar, StatusType},
    constants::{
        DEFENSE_BAR_LIMIT, DEFENSE_BAR_X, DEFENSE_BAR_Y, HEALTH_BAR_LIMIT, HEALTH_BAR_X,
        HEALTH_BAR_Y, RESTOCK_BAR_LIMIT, RESTOCK_BAR_X, RESTOCK_BAR_Y, ROLL_BAR_LIMIT, ROLL_BAR_X,
        ROLL_BAR_Y,
    },
};
use amethyst::{
    ecs::{World, WorldExt},
    prelude::Builder,
};

pub fn initialise_status_bars(world: &mut World) {
    //health bar
    world
        .create_entity()
        .with(StatusBar {
            status_type: StatusType::Health,
            x_pos: HEALTH_BAR_X,
            y_pos: HEALTH_BAR_Y,
            status_unit_stack: vec![],
            unit_limit: HEALTH_BAR_LIMIT,
        })
        .build();

    //defense bar
    world
        .create_entity()
        .with(StatusBar {
            status_type: StatusType::Defense,
            x_pos: DEFENSE_BAR_X,
            y_pos: DEFENSE_BAR_Y,
            status_unit_stack: vec![],
            unit_limit: DEFENSE_BAR_LIMIT,
        })
        .build();

    //roll bar
    world
        .create_entity()
        .with(StatusBar {
            status_type: StatusType::Roll,
            x_pos: ROLL_BAR_X,
            y_pos: ROLL_BAR_Y,
            status_unit_stack: vec![],
            unit_limit: ROLL_BAR_LIMIT,
        })
        .build();

    //restock bar
    world
        .create_entity()
        .with(StatusBar {
            status_type: StatusType::Restock,
            x_pos: RESTOCK_BAR_X,
            y_pos: RESTOCK_BAR_Y,
            status_unit_stack: vec![],
            unit_limit: RESTOCK_BAR_LIMIT,
        })
        .build();
}
