use std::collections::HashMap;

mod audio;
mod play_audio;

pub use self::{
    audio::{initialize_audio, play_sfx, Sounds},
    play_audio::PlayAudioSystem,
};

pub type SoundsConfig = HashMap<String, String>;
