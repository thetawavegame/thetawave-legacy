//! Components for spawning and despawning entities

mod child_spawner;
mod despawn;

pub use self::{
    child_spawner::{
        AutoConsumableSpawnerComponent, AutoEffectSpawnerComponent, AutoItemSpawnerComponent,
        AutoMobSpawnerComponent, AutoSpawnerComponent,
    },
    despawn::{DespawnAtBorderComponent, DespawnTimeLimitComponent},
};
