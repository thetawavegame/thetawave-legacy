//! Resources for managing visuals

use amethyst::{assets::Handle, renderer::SpriteSheet};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SpriteRenderData {
    pub initial_index: usize,
    pub spritesheet: String,
}

#[derive(Clone)]
pub struct SpriteSheetsResource {
    pub spritesheets: HashMap<String, Handle<SpriteSheet>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SpriteSheetData {
    pub image: String,
    pub data: String,
}

pub type SpriteSheetsConfig = HashMap<String, SpriteSheetData>;
