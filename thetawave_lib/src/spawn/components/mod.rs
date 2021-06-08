mod child_spawner;
mod despawn;
mod timelimit;

pub use self::{
    child_spawner::{
        AutoConsumableSpawnerComponent, AutoEffectSpawnerComponent, AutoItemSpawnerComponent,
        AutoMobSpawnerComponent, AutoSpawnerComponent,
    },
    despawn::DespawnAtBorderComponent,
    timelimit::TimeLimitComponent,
};
