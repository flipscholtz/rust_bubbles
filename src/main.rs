use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::structs::GameState;
use ggez::event::{self};
use ggez::{ContextBuilder, conf};

mod constants;
mod game;
mod renderer;
mod structs;

fn main() {
    let (ctx, event_loop) = ContextBuilder::new("NumberCatcher", "Flippie Scholtz")
        .window_setup(conf::WindowSetup::default().title("Rust Bubbles"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()
        .expect("error, could not create ggez context!");
    let game = GameState::new(&ctx);

    event::run(ctx, event_loop, game);
}
