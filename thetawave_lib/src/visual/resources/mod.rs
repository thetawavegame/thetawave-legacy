//! Resources for managing visuals

use amethyst::{assets::Handle, renderer::SpriteSheet};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Stores initial index and of sprite and sprite sheet name
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SpriteRenderData {
    pub initial_index: usize,
    pub spritesheet: String,
}

/// Map names of sprite sheets to sprite sheet handles
#[derive(Clone)]
pub struct SpriteSheetsResource {
    pub spritesheets: HashMap<String, Handle<SpriteSheet>>,
}

/// Stores paths to sprite sheet image files and data (ron) files
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SpriteSheetData {
    pub image: String,
    pub data: String,
}

/// Map names of sprite sheets to SpriteSheetData
pub type SpriteSheetsConfig = HashMap<String, SpriteSheetData>;
