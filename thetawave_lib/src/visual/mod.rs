//! `thetawave_lib` visual module

mod components;
mod entities;
mod resources;
mod systems;

pub use self::{
    components::{
        animation::{AnimationComponent, AnimationType},
        fade::{ColorChannelChange, FadeComponent},
        planet::PlanetComponent,
        status_bar::{StatusBarComponent, StatusType},
    },
    entities::{
        initialize_background, initialize_planet, initialize_side_panels, initialize_status_bars,
        spawn_status_unit,
    },
    resources::{SpriteRenderData, SpriteSheetData, SpriteSheetsConfig, SpriteSheetsResource},
    systems::{
        AnimationSystem, FadeSystem, PlanetsSystem, StatTrackerSystem, StatusBarSystem,
        TrackedStats,
    },
};
