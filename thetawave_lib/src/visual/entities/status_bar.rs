use crate::{
    constants::{
        DEFENSE_BAR_LIMIT, DEFENSE_BAR_X, DEFENSE_BAR_Y, HEALTH_BAR_LIMIT, HEALTH_BAR_X,
        HEALTH_BAR_Y, RESTOCK_BAR_LIMIT, RESTOCK_BAR_X, RESTOCK_BAR_Y, ROLL_BAR_LIMIT, ROLL_BAR_X,
        ROLL_BAR_Y,
    },
    visual::{StatusBarComponent, StatusType},
};
use amethyst::{
    core::math::Vector2,
    ecs::{World, WorldExt},
    prelude::Builder,
};

/// Initialize the health, defense, roll, and store status bars
pub fn initialize_status_bars(world: &mut World) {
    //health bar
    world
        .create_entity()
        .with(StatusBarComponent {
            status_type: StatusType::Health,
            position: Vector2::new(HEALTH_BAR_X, HEALTH_BAR_Y),
            status_unit_stack: vec![],
            unit_limit: HEALTH_BAR_LIMIT,
        })
        .build();

    //defense bar
    world
        .create_entity()
        .with(StatusBarComponent {
            status_type: StatusType::Defense,
            position: Vector2::new(DEFENSE_BAR_X, DEFENSE_BAR_Y),
            status_unit_stack: vec![],
            unit_limit: DEFENSE_BAR_LIMIT,
        })
        .build();

    //roll bar
    world
        .create_entity()
        .with(StatusBarComponent {
            status_type: StatusType::Roll,
            position: Vector2::new(ROLL_BAR_X, ROLL_BAR_Y),
            status_unit_stack: vec![],
            unit_limit: ROLL_BAR_LIMIT,
        })
        .build();

    //restock bar
    world
        .create_entity()
        .with(StatusBarComponent {
            status_type: StatusType::Restock,
            position: Vector2::new(RESTOCK_BAR_X, RESTOCK_BAR_Y),
            status_unit_stack: vec![],
            unit_limit: RESTOCK_BAR_LIMIT,
        })
        .build();
}
