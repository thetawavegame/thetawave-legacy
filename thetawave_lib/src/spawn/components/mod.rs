//! Components for spawning and despawning entities

mod despawn;
mod spawn;

pub use self::{
    despawn::{DespawnAtBorderComponent, DespawnTimeLimitComponent},
    spawn::{
        AutoConsumableSpawnerComponent, AutoEffectSpawnerComponent, AutoItemSpawnerComponent,
        AutoMobSpawnerComponent, AutoSpawnerComponent,
    },
};
