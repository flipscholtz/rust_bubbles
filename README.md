# Rust bubbles
A simple game: Catch numbered bubbles with your ship to make up a target sum.

Uses the ggez game engine for rendering and input handling.

## Running
`cargo run`

## Settings
constants.rs contains various settings related to the difficulty and appearance of the game, although there are still some magic numbers dotted about the code.

## Code
- All game state lives in the GameState struct ( structs.rs ).
- Game logic lives in GameState implementation ( game.rs ).
- Game state is rendered to screen by renderer.rs .

On every tick, ggez calls 'update' followed by 'draw', both on GameState's implementation of EventHandler.

## Known issues
Nothing is rendered with the latest version of ggez. Downgrading to 0.8.5 fixes it.
https://github.com/ggez/ggez/issues/1185