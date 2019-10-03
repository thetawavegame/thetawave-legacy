// game dimensions
pub const GAME_WIDTH: f32 = 360.0;
pub const GAME_HEIGHT: f32 = 270.0;

// arena dimensions, area excluding the side bars
pub const ARENA_MIN_Y: f32 = 0.0;
pub const ARENA_MAX_Y: f32 = GAME_HEIGHT;
pub const ARENA_MIN_X: f32 = GAME_WIDTH / 8.0;
pub const ARENA_MAX_X: f32 = GAME_WIDTH - ARENA_MIN_X;
pub const ARENA_WIDTH: f32 = ARENA_MAX_X - ARENA_MIN_X;
pub const ARENA_HEIGHT: f32 = ARENA_MAX_Y - ARENA_MIN_Y;
pub const ARENA_SPAWN_OFFSET: f32 = 20.0;

// camera
pub const CAMERA_X: f32 = GAME_WIDTH * 0.5;
pub const CAMERA_Y: f32 = GAME_HEIGHT * 0.5;
pub const CAMERA_Z: f32 = 237.0;

// Z levels of sprites
pub const ENEMY_Z: f32 = 0.0;
pub const PLAYER_Z: f32 = 0.0;
pub const CONSUMABLE_Z: f32 = 0.0;
pub const ITEM_Z: f32 = 0.0;
pub const BLAST_Z: f32 = 0.9;
pub const EXPLOSION_Z: f32 = 0.0;

// stats
pub const DEFENSE: f32 = 500.0;

