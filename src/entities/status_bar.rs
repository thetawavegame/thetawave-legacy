use amethyst::{
    prelude::Builder,
    ecs::{World, WorldExt},
};
use crate::{
    components::{StatusBar, StatusType},
};

const HEALTH_X: f32 = 332.0;
const HEALTH_Y: f32 = 200.0;
const HEALTH_LIMIT: f32 = 63.0;
const DEFENSE_X: f32 = 352.0;
const DEFENSE_Y: f32 = 200.0;
const DEFENSE_LIMIT: f32 = 63.0;
const ROLL_X: f32 = 324.0;
const ROLL_Y: f32 = 177.0;
const ROLL_LIMIT: f32 = 28.0;
const RESTOCK_X: f32 = 324.0;//should appear in arena
const RESTOCK_Y: f32 = 90.0;
const RESTOCK_LIMIT: f32 = 28.0;

pub fn initialise_status_bars(world: &mut World) {

    //health bar
    world
        .create_entity()
        .with(StatusBar{
            status_type: StatusType::Health,
            x_pos: HEALTH_X,
            y_pos: HEALTH_Y,
            status_unit_stack: vec![],
            unit_limit: HEALTH_LIMIT,
        })
        .build();

    //defense bar
    world
        .create_entity()
        .with(StatusBar{
            status_type: StatusType::Defense,
            x_pos: DEFENSE_X,
            y_pos: DEFENSE_Y,
            status_unit_stack: vec![],
            unit_limit: DEFENSE_LIMIT,
        })
        .build();

    //roll bar
    world
        .create_entity()
        .with(StatusBar{
            status_type: StatusType::Roll,
            x_pos: ROLL_X,
            y_pos: ROLL_Y,
            status_unit_stack: vec![],
            unit_limit: ROLL_LIMIT,
        })
        .build();

    //restock bar
    world
        .create_entity()
        .with(StatusBar {
            status_type: StatusType::Restock,
            x_pos: RESTOCK_X,
            y_pos: RESTOCK_Y,
            status_unit_stack: vec![],
            unit_limit: RESTOCK_LIMIT,
        })
        .build();

}