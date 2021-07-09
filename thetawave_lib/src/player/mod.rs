//! `thetawave_lib` player module

mod components;
mod entities;
mod resources;
mod systems;

pub use self::{
    components::{AbilityDirection, BarrelRollAbilityComponent, CooldownAbility, PlayerComponent},
    entities::initialize_spaceship,
    resources::{PlayerEntityData, PlayersResource},
    systems::BarrelRollAbilitySystem,
};
