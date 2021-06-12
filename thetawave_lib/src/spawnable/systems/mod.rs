//! Systems for spawnable entities

mod mob;
mod modifiers;

pub use self::{
    mob::{MobBehaviorSystem, MobDestroyedSystem},
    modifiers::ModifiersSystem,
};
