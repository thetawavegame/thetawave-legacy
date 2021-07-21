//! `thetawave_lib` constants module

// game dimensions
pub const GAME_WIDTH: f32 = 360.0;
pub const GAME_HEIGHT: f32 = 270.0;

// arena dimensions, area excluding the sidebars
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
pub const BOSS_Z_1: f32 = 0.1;
pub const BOSS_Z_2: f32 = 0.2;
pub const BLAST_Z: f32 = 0.9;
pub const STATUS_BAR_Z: f32 = 0.9;
pub const SIDE_PANEL_Z: f32 = 0.8;

// sprites
pub const PLAYER_BLAST_SPRITE_INDEX: usize = 0;
pub const ENEMY_BLAST_SPRITE_INDEX: usize = 1;
pub const CRIT_BLAST_SPRITE_INDEX: usize = 2;
pub const POISON_BLAST_SPRITE_INDEX: usize = 3;
pub const SIDE_PANEL_LEFT_SPRITE_INDEX: usize = 0;
pub const SIDE_PANEL_RIGHT_SPRITE_INDEX: usize = 1;
pub const SPACESHIP_BLAST_SPRITE_INDEX: usize = 0;

// dimensions
pub const BLAST_HITBOX_DIAMETER: f32 = 2.0;
pub const SIDE_PANEL_WIDTH: f32 = 45.0;

// offsets
pub const ITEM_SPAWN_Y_OFFSET: f32 = 20.0;
pub const SPAWNER_Y_OFFSET: f32 = 20.0;

// player starting stats
pub const SPACESHIP_HITBOX_HEIGHT: f32 = 13.0;
pub const SPACESHIP_HITBOX_WIDTH: f32 = 6.0;
pub const SPACESHIP_ACCELERATION_X: f32 = 2.0;
pub const SPACESHIP_DECELERATION_X: f32 = 1.7;
pub const SPACESHIP_ACCELERATION_Y: f32 = 3.0;
pub const SPACESHIP_DECELERATION_Y: f32 = 2.5;
pub const SPACESHIP_MAX_SPEED: f32 = 70.0;
pub const SPACESHIP_MAX_KNOCKBACK_SPEED: f32 = 100.0;
pub const SPACESHIP_FIRE_SPEED: f32 = 0.3;
pub const SPACESHIP_DAMAGE: f32 = 40.0;
pub const SPACESHIP_BARREL_COOLDOWN: f32 = 2.0;
pub const SPACESHIP_BARREL_SPEED: f32 = 180.0;
pub const SPACESHIP_BARREL_DURATION: f32 = 0.3;
pub const SPACESHIP_HEALTH: f32 = 400.0;

// status bar
pub const HEALTH_BAR_X: f32 = 332.0;
pub const HEALTH_BAR_Y: f32 = 200.0;
pub const HEALTH_BAR_LIMIT: f32 = 63.0;
pub const DEFENSE_BAR_X: f32 = 352.0;
pub const DEFENSE_BAR_Y: f32 = 200.0;
pub const DEFENSE_BAR_LIMIT: f32 = 63.0;
pub const ROLL_BAR_X: f32 = 324.0;
pub const ROLL_BAR_Y: f32 = 177.0;
pub const ROLL_BAR_LIMIT: f32 = 28.0;
pub const RESTOCK_BAR_X: f32 = 324.0;
pub const RESTOCK_BAR_Y: f32 = 90.0;
pub const RESTOCK_BAR_LIMIT: f32 = 28.0;
