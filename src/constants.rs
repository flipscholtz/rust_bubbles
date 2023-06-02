use ggez::glam::Vec2;

// Gameplay:

pub const MIN_TARGET: u32 = 5;
pub const MAX_TARGET: u32 = 75;
pub const STARTING_LIVES: u8 = 3;

pub const MIN_BUBBLE_SPEED: f32 = 1.0;
pub const MAX_BUBBLE_SPEED: f32 = 2.9;
pub const SHIP_SPEED: Vec2 = Vec2::new(10.0, 8.0);

pub const RESPECT_SHORTFALL_PROBABILITY: f64 = 0.3;
pub const NEW_BUBBLE_INTERVAL_FRAMES: usize = 30;

pub const STARTING_ROUND_TIME_SECONDS: u64 = 45;
pub const MIN_ROUND_TIME_SECONDS: u64 = 15;
pub const TIME_DEDUCTED_PER_ROUND: u64 = 5;

pub const SECONDS_LEFT_PER_BONUS_POINT: u64 = 2;

// Visual:
pub const WINDOW_WIDTH: f32 = 1024.0;
pub const WINDOW_HEIGHT: f32 = 768.0;
pub const BUBBLE_RADIUS: f32 = 30.0;
