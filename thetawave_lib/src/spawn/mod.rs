//! `thetawave_lib` spawn module

mod components;
mod resources;
mod systems;

pub use self::{
    components::{
        AutoConsumableSpawnerComponent, AutoEffectSpawnerComponent, AutoItemSpawnerComponent,
        AutoMobSpawnerComponent, AutoSpawnerComponent, DespawnAtBorderComponent,
        DespawnTimeLimitComponent,
    },
    resources::{
        DropProbabilities, DropRolls, DropTableType, DropTablesResource, Formation,
        FormationSpawnable, RandomSpawnable, RollProbabilities, SpawnerResource,
    },
    systems::{AutoSpawnerSystem, DespawnAtBorderSystem, DespawnTimeLimitSystem, SpawnerSystem},
};
