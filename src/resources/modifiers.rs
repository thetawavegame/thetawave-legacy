use crate::entities::{ConsumableType, ItemType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type ItemModifiersResource = HashMap<ItemType, Vec<Modifier>>;
pub type ConsumableModifiersResource = HashMap<ConsumableType, Vec<Modifier>>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Modifier {
    BarrelImmunity(bool),
    ProjectileFirePeriod(f32),
    ProjectileDamage(f32),
    MaximumSpeed(f32),
    Deceleration(f32),
    Acceleration(f32),
    CriticalDamageChance(f32),
    PoisonChance(f32),
    MaximumDefense(f32),
    Defense(f32),
    ProjectileCount(usize),
    AbilityCooldown(f32),
    MaximumHealth(f32),
    Health(f32),
    ProjectileSize(f32),
    Armor(usize),
    Money(usize),
}
