use ggez::{glam::Vec2, winit::dpi::PhysicalSize};

#[derive(Debug)]
pub enum GameMode {
    IntroScreen,
    Running,
    NextRoundScreen,
    OvershotScreen,
    OutOfTimeScreen,
    WinScreen,
    DeathScreen,
}

#[derive(Debug)]
pub struct GameState {
    pub window_size: PhysicalSize<u32>,
    pub current_mode: GameMode,
    pub current_round: u32,
    pub round_start_time_seconds: u64,
    pub round_allowed_time_seconds: u64,
    pub round_time_remaining_seconds: u64,
    pub lives_remaining: u8,
    pub current_target: u32,
    pub numbers_caught: Vec<u32>,
    pub score: u32,
    pub round_time_bonus: u32,
    pub ship: Ship,
    pub bubbles: Vec<Bubble>,
    pub next_bubble_index: u32,
}

#[derive(Debug)]
pub struct Bubble {
    pub index: u32,
    pub number: u32,
    pub position: Vec2,
    pub speed: Vec2,
}

#[derive(Debug)]
pub struct Ship {
    pub position: Vec2,
    pub speed: Vec2,
}
