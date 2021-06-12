//! Components unique to spawnable entities

mod blast;
mod consumable;
mod item;
mod mob;

pub use self::{
    blast::BlastComponent, consumable::ConsumableComponent, item::ItemComponent, mob::MobComponent,
};
