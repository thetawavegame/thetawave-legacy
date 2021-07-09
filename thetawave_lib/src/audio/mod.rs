//! `thetawave_lib` motion module

use std::collections::HashMap;

mod audio_utils;
mod play_audio;

pub use self::{
    audio_utils::{initialize_audio, play_sfx, Sounds},
    play_audio::PlayAudioSystem,
};

/// Maps sound effect names to paths
pub type SoundsConfig = HashMap<String, String>;
