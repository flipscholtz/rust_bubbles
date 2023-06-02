use std::cmp::Ordering;

use ggez::event::EventHandler;
use ggez::glam::Vec2;
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard::KeyInput;
use ggez::{Context, GameResult};
use rand::{self, prelude::*};

use crate::constants;
use crate::constants::STARTING_LIVES;
use crate::renderer;
use crate::structs::{Bubble, GameMode, GameState, Ship};

impl GameState {
    pub fn new(ctx: &Context) -> Self {
        Self {
            window_size: ctx.gfx.window().inner_size(),
            lives_remaining: constants::STARTING_LIVES,
            current_mode: GameMode::IntroScreen,
            current_round: 0,
            round_allowed_time_seconds: constants::STARTING_ROUND_TIME_SECONDS,
            round_start_time_seconds: ctx.time.time_since_start().as_secs(),
            round_time_remaining_seconds: constants::STARTING_ROUND_TIME_SECONDS,
            bubbles: vec![],
            next_bubble_index: 0,
            current_target: 0,
            numbers_caught: vec![],
            score: 0,
            round_time_bonus: 0,
            ship: Ship {
                position: Vec2::new(500.0, 500.0),
                speed: constants::SHIP_SPEED,
            },
            paused: false,
        }
    }

    pub fn random_between(lower: f32, upper: f32) -> f32 {
        let mut rng: ThreadRng = rand::thread_rng();
        let window: f32 = upper - lower;
        return lower + ((rng.gen::<f64>()) * window as f64) as f32;
    }

    pub fn random_target() -> u32 {
        return Self::random_between(constants::MIN_TARGET as f32, constants::MAX_TARGET as f32)
            as u32;
    }

    pub fn overlaps_with_bubble(bubbles: &Vec<Bubble>, pos: Vec2) -> Option<&Bubble> {
        for bubble in bubbles.iter() {
            let overlaps_x: bool = pos.x >= bubble.position.x - constants::BUBBLE_RADIUS * 2.0
                && pos.x <= bubble.position.x + constants::BUBBLE_RADIUS * 2.0;
            let overlaps_y: bool = pos.y >= bubble.position.y - constants::BUBBLE_RADIUS
                && pos.y <= bubble.position.y + constants::BUBBLE_RADIUS;
            if overlaps_x && overlaps_y {
                return Some(bubble);
            }
        }
        return None;
    }

    pub fn add_bubble(&mut self) {
        /*
         * To balance making the game challenging but not too hard,
         * 50% of the time we make sure the bubble is in the same range as the current shortfall,
         * so you're not stuck needing a 2 and having to wait through random numbers between 1 and 100 for e.g.
         */

        let mut rng = rand::thread_rng();
        let should_respect_shortfall_range: bool =
            rng.gen::<f64>() < constants::RESPECT_SHORTFALL_PROBABILITY;

        let mut upper_limit: u32 = self.current_target;
        if should_respect_shortfall_range {
            let shortfall = self.current_target - self.compute_caught_sum();
            upper_limit = shortfall;
        }
        let bubble_number = Self::random_between(1.0, upper_limit as f32) as u32;

        let mut bubble_x: f32 = Self::random_between(1.0, (self.window_size.width - 10) as f32);
        while Option::is_some(&Self::overlaps_with_bubble(
            &self.bubbles,
            Vec2::new(bubble_x as f32, 0.0),
        )) {
            bubble_x = Self::random_between(10.0, (self.window_size.width - 10) as f32);
        }
        let bubble_y = 0;
        let bubble: Bubble = Bubble {
            index: self.next_bubble_index,
            number: bubble_number,
            position: Vec2::new(bubble_x as f32, bubble_y as f32),
            speed: Vec2::new(
                0.0,
                Self::random_between(
                    constants::MIN_BUBBLE_SPEED + (self.current_round as f32 / 2.0),
                    constants::MAX_BUBBLE_SPEED + (self.current_round as f32 / 2.0),
                ),
            ),
        };
        self.bubbles.push(bubble);
        self.next_bubble_index += 1;
    }

    fn update_bubbles(&mut self) {
        for bubble in self.bubbles.iter_mut() {
            bubble.position.y += bubble.speed.y;
            bubble.position.x += bubble.speed.x;
        }
    }

    fn check_bubble_caught(&mut self) {
        match Self::overlaps_with_bubble(&self.bubbles, self.ship.position) {
            Some(bubble) => {
                self.numbers_caught.push(bubble.number);
                let index_to_remove = self
                    .bubbles
                    .iter()
                    .position(|b| (*b).index == bubble.index)
                    .unwrap();
                self.bubbles.remove(index_to_remove);
            }
            None => (),
        }
    }

    pub fn compute_caught_sum(&self) -> u32 {
        let mut total: u32 = 0;
        for i in self.numbers_caught.iter() {
            total += i;
        }
        return total;
    }

    fn handle_input(&mut self, ctx: &mut Context) {
        match self.current_mode {
            GameMode::Running => {
                if ctx.keyboard.is_key_pressed(KeyCode::Left) && self.ship.position.x >= 5.0 {
                    self.ship.position.x -= self.ship.speed.x;
                }
                if ctx.keyboard.is_key_pressed(KeyCode::Right)
                    && self.ship.position.x <= (self.window_size.width - 15) as f32
                {
                    self.ship.position.x += self.ship.speed.x;
                }
                if ctx.keyboard.is_key_pressed(KeyCode::Up) && self.ship.position.y >= 0.0 {
                    self.ship.position.y -= self.ship.speed.y;
                }
                if ctx.keyboard.is_key_pressed(KeyCode::Down)
                    && self.ship.position.y <= (self.window_size.height - 25) as f32
                {
                    self.ship.position.y += self.ship.speed.y;
                }
            }
            _ => ()
        }
    }

    fn process_timer(&mut self, ctx: &Context) {
       // println!("Time: {}",ctx.time.time_since_start().as_millis() );
        let time_elapsed = ctx.time.time_since_start().as_secs() - self.round_start_time_seconds;
        self.round_time_remaining_seconds = self.round_allowed_time_seconds - time_elapsed;
        if self.round_time_remaining_seconds <= 0 {
            self.deduct_life();
            self.current_mode = GameMode::OutOfTimeScreen;
            self.round_time_remaining_seconds = 0;
        }
    }

    fn deduct_life(&mut self) {
        if self.lives_remaining > 0 {
            self.lives_remaining -= 1;
        }
    }

    fn handle_win(&mut self) {
        self.score += 1;
        self.round_time_bonus = (self.round_time_remaining_seconds / constants::SECONDS_LEFT_PER_BONUS_POINT) as u32;
        self.score += self.round_time_bonus;
        self.current_mode = GameMode::WinScreen;
    }

    fn prepare_next_round(&mut self) {
        self.current_target = Self::random_target();
        self.bubbles = vec![];
        self.numbers_caught = vec![];

        self.round_time_remaining_seconds = self.round_allowed_time_seconds;
        self.current_round += 1;
        if self.round_allowed_time_seconds > constants::MIN_ROUND_TIME_SECONDS {
            self.round_allowed_time_seconds -= constants::TIME_DEDUCTED_PER_ROUND;
        }
        
        self.round_time_bonus = 0;
        self.current_mode = GameMode::NextRoundScreen;
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.handle_input(ctx);

        if self.paused || !matches!(self.current_mode, GameMode::Running) {
            return Ok(());
        }

        self.process_timer(ctx);

        if ctx.time.ticks() % constants::NEW_BUBBLE_INTERVAL_FRAMES == 0 {
            self.add_bubble();
        }

        self.update_bubbles();
        self.check_bubble_caught();

        let current_sum: u32 = self.compute_caught_sum();

        match current_sum.cmp(&self.current_target) {
            Ordering::Less => (),
            Ordering::Greater => {
                // Overshot
                self.deduct_life();
                self.current_mode = GameMode::OvershotScreen;
            }
            Ordering::Equal => {
                // Win!
               self.handle_win();
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        return renderer::render(self, ctx);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeat: bool,
    ) -> GameResult {
        match self.current_mode {
            GameMode::IntroScreen => {
                // Spacebar goes to the 'next round' screen:
                if input.keycode.unwrap() == KeyCode::Space {
                    self.prepare_next_round();
                }
            },
            GameMode::OvershotScreen | GameMode::OutOfTimeScreen => {
                // Spacebar goes to the 'next round' screen, or Death if no more lives left.
                if input.keycode.unwrap() == KeyCode::Space {
                    if self.lives_remaining <= 0 {
                        self.current_mode = GameMode::DeathScreen;
                    } else {
                        self.prepare_next_round();
                    }
                }
            },
            GameMode::DeathScreen => {
                if input.keycode.unwrap() == KeyCode::Space {
                    self.score = 0;
                    self.lives_remaining = STARTING_LIVES;
                    self.current_mode = GameMode::IntroScreen;
                }
            },
            GameMode::WinScreen => {
                if input.keycode.unwrap() == KeyCode::Space {
                    self.prepare_next_round();
                }
            },
            GameMode::Running => {
                if input.keycode.unwrap() == KeyCode::P {
                    self.paused = !self.paused;
                }
            },
            _ => {
                // Spacebar starts the game:
                if input.keycode.unwrap() == KeyCode::Space {
                    self.current_mode = GameMode::Running;
                    self.round_start_time_seconds = ctx.time.time_since_start().as_secs();
                }
            }
        }
        /*
        println!(
            "Key pressed: scancode {}, keycode {:?}, modifier {:?}, repeat: {}",
            input.scancode, input.keycode, input.mods, repeat
        );
        */
        Ok(())
    }
}
