mod main_game;
mod paused;

pub use self::{
    main_game::{MainGameState, TrackedStats},
    paused::PausedState,
};
