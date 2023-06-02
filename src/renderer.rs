use crate::constants;
use crate::structs::{GameMode, GameState};
use ggez::glam::Vec2;
use ggez::graphics::{self, Canvas, Color, DrawParam, PxScale, Text, TextFragment};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

pub fn render(game: &GameState, ctx: &mut Context) -> GameResult {
    let mut canvas: Canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

    match game.current_mode {
        GameMode::IntroScreen => draw_intro_screen(ctx, &mut canvas, game),
        GameMode::OvershotScreen => draw_overshot_screen(ctx, &mut canvas, game),
        GameMode::OutOfTimeScreen => draw_out_of_time_screen(ctx, &mut canvas, game),
        GameMode::WinScreen => draw_win_screen(ctx, &mut canvas, game),
        GameMode::DeathScreen => draw_death_screen(ctx, &mut canvas, game),
        GameMode::NextRoundScreen => draw_next_round_screen(ctx, &mut canvas, game),
        GameMode::Running => {
           // draw_target_number(&mut canvas, game.current_target);
            draw_ship(ctx, &mut canvas, game);
            draw_bubbles(ctx, &mut canvas, game);
            draw_current_total(ctx, game, &mut canvas);
            draw_score(game, &mut canvas);
            draw_lives(ctx, &mut canvas, game);
            draw_remaining_time(ctx, &mut canvas, game);
        }
    }
    return canvas.finish(ctx);
}

fn draw_current_total(_ctx: &Context, game: &GameState, canvas: &mut Canvas) {
    // Text:
    let text = Text::new(TextFragment {
        text: format!("SUM: {}/{}", game.compute_caught_sum(), game.current_target),
        color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
        font: Some("LiberationMono-Regular".into()),
        scale: Some(PxScale::from(65.0)),
    });
    canvas.draw(
        &text,
        DrawParam::from(Vec2::new((10) as f32, 20.0)),
    );
}

fn draw_remaining_time(_ctx: &Context, canvas: &mut Canvas, game: &GameState) {
    // Text:
    let color: Color;
    match game.round_time_remaining_seconds {
        0..=5 => color = Color::RED,
        6..=10 => color = Color::YELLOW,
        _ => color = Color::BLUE,
    }
    let text = Text::new(TextFragment {
        text: format!("TIME: {}/{}", game.round_time_remaining_seconds, game.round_allowed_time_seconds),
        color: Some(color),
        font: Some("LiberationMono-Regular".into()),
        scale: Some(PxScale::from(65.0)),
    });

    canvas.draw(
        &text,
        DrawParam::from(Vec2::new((game.window_size.width - 420) as f32, 20.0)),
    );
}

fn draw_score(game: &GameState, canvas: &mut Canvas) {
    let text = Text::new(TextFragment {
        text: format!("Score: {}", game.score),
        color: Some(Color::new(0.0, 0.0, 1.0, 1.0)),
        font: Some("LiberationMono-Regular".into()),
        scale: Some(PxScale::from(30.0)),
    });
    canvas.draw(
        &text,
        DrawParam::from(Vec2::new((game.window_size.width - 300) as f32, 100.0)),
    );
}

fn draw_ship(ctx: &Context, canvas: &mut Canvas, game: &GameState) {
    let pos = game.ship.position;

    let triangle_points: [Point2<f32>; 4] = [
        Point2::from(pos),
        Point2::from(Vec2::new(pos.x + 20.0, pos.y + 30.0)),
        Point2::from(Vec2::new(pos.x - 20.0, pos.y + 30.0)),
        Point2::from(pos),
    ];
    let ship_mesh = graphics::Mesh::new_polyline(
        ctx,
        graphics::DrawMode::stroke(1.0),
        &triangle_points,
        Color::BLUE,
    )
    .expect("Could not build ship mesh");

    canvas.draw(&ship_mesh, DrawParam::default());
}

fn draw_lives(ctx: &Context, canvas: &mut Canvas, game: &GameState) {
    let text = Text::new(TextFragment {
        text: format!("Lives: "),
        color: Some(Color::new(0.0, 1.0, 0.0, 1.0)),
        font: Some("LiberationMono-Regular".into()),
        scale: Some(PxScale::from(30.0)),
    });

    canvas.draw(
        &text,
        DrawParam::from(Vec2::new(10.0, 80.0)),
    );

    for i in 1..=game.lives_remaining {
        let dot_mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Point2::from(Vec2::new(
                70.0 + (60.0 * i as f32),
                100.0,
            )),
            5.0,
            1.0,
            Color::GREEN,
        )
        .expect("error creating bubble mesh");

        canvas.draw(&dot_mesh, DrawParam::default());
    }
}

fn draw_bubbles(ctx: &Context, canvas: &mut Canvas, game: &GameState) {
    let color: Color = Color::from_rgb(71, 252, 222);
    for bubble in game.bubbles.iter() {
        let bubble_mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::stroke(2.0),
            Point2::from(bubble.position),
            constants::BUBBLE_RADIUS,
            1.0,
            color,
        )
        .expect("error creating bubble mesh");

        canvas.draw(&bubble_mesh, DrawParam::default());

        let text = Text::new(TextFragment {
            text: format!("{}", bubble.number),
            color: Some(color),
            font: Some("LiberationMono-Regular".into()),
            scale: Some(PxScale::from(30.0)),
        });

        canvas.draw(
            &text,
            DrawParam::from(bubble.position - Vec2::new(12.0, 10.0)),
        );
    }
}

fn draw_overshot_screen(_ctx: &Context, canvas: &mut Canvas, game: &GameState) {
    let overshot_text = Text::new(TextFragment {
        text: format!("OVERSHOT!\nLives left: {}", game.lives_remaining),
        color: Some(Color::RED),
        font: Some("LiberationMono-Regular".into()),
        scale: Some(PxScale::from(100.0)),
    });

    let press_space_text = Text::new(TextFragment {
        text: format!("Press space to continue..."),
        color: Some(Color::RED),
        font: Some("LiberationMono-Regular".into()),
        scale: Some(PxScale::from(50.0)),
    });

    canvas.draw(
        &overshot_text,
        DrawParam::from(Vec2::new(
            game.window_size.width as f32 / 7.0,
            game.window_size.height as f32 / 3.0,
        )),
    );

    canvas.draw(
        &press_space_text,
        DrawParam::from(Vec2::new(
            game.window_size.width as f32 / 7.0,
            game.window_size.height as f32 / 3.0 + 200.0,
        )),
    );
}

fn draw_out_of_time_screen(_ctx: &Context, canvas: &mut Canvas, game: &GameState) {
    let overshot_text = Text::new(TextFragment {
        text: format!("OUT OF TIME!\nLives left: {}", game.lives_remaining),
        color: Some(Color::RED),
        font: Some("LiberationMono-Regular".into()),
        scale: Some(PxScale::from(100.0)),
    });

    let press_space_text = Text::new(TextFragment {
        text: format!("Press space to continue..."),
        color: Some(Color::RED),
        font: Some("LiberationMono-Regular".into()),
        scale: Some(PxScale::from(50.0)),
    });

    canvas.draw(
        &overshot_text,
        DrawParam::from(Vec2::new(
            game.window_size.width as f32 / 7.0,
            game.window_size.height as f32 / 3.0,
        )),
    );

    canvas.draw(
        &press_space_text,
        DrawParam::from(Vec2::new(
            game.window_size.width as f32 / 7.0,
            game.window_size.height as f32 / 3.0 + 200.0,
        )),
    );
}

fn draw_death_screen(_ctx: &Context, canvas: &mut Canvas, game: &GameState) {
    let death_text = Text::new(TextFragment {
        text: format!("NO MORE LIVES!\nYour score: {}", game.score),
        color: Some(Color::RED),
        font: Some("LiberationMono-Regular".into()),
        scale: Some(PxScale::from(100.0)),
    });

    let press_space_text = Text::new(TextFragment {
        text: format!("Press space to play again..."),
        color: Some(Color::RED),
        font: Some("LiberationMono-Regular".into()),
        scale: Some(PxScale::from(50.0)),
    });

    canvas.draw(
        &death_text,
        DrawParam::from(Vec2::new(
            game.window_size.width as f32 / 7.0,
            game.window_size.height as f32 / 3.0,
        )),
    );

    canvas.draw(
        &press_space_text,
        DrawParam::from(Vec2::new(
            game.window_size.width as f32 / 7.0,
            game.window_size.height as f32 / 3.0 + 200.0,
        )),
    );
}

fn draw_next_round_screen(_ctx: &Context, canvas: &mut Canvas, game: &GameState) {
    let new_target_text = Text::new(TextFragment {
        text: format!("NEW TARGET: {}", game.current_target),
        color: Some(Color::GREEN),
        font: Some("LiberationMono-Regular".into()),
        scale: Some(PxScale::from(100.0)),
    });

    let press_space_text = Text::new(TextFragment {
        text: format!("Press space to continue..."),
        color: Some(Color::GREEN),
        font: Some("LiberationMono-Regular".into()),
        scale: Some(PxScale::from(50.0)),
    });

    canvas.draw(
        &new_target_text,
        DrawParam::from(Vec2::new(
            game.window_size.width as f32 / 7.0,
            game.window_size.height as f32 / 3.0,
        )),
    );

    canvas.draw(
        &press_space_text,
        DrawParam::from(Vec2::new(
            game.window_size.width as f32 / 7.0,
            game.window_size.height as f32 / 3.0 + 100.0,
        )),
    );
}

fn draw_win_screen(_ctx: &Context, canvas: &mut Canvas, game: &GameState) {
    let new_target_text = Text::new(TextFragment {
        text: format!("NOICE!\nNew score: {}\n(Time bonus: {})", game.score, game.round_time_bonus),
        color: Some(Color::GREEN),
        font: Some("LiberationMono-Regular".into()),
        scale: Some(PxScale::from(100.0)),
    });

    let press_space_text = Text::new(TextFragment {
        text: format!("Press space to continue..."),
        color: Some(Color::GREEN),
        font: Some("LiberationMono-Regular".into()),
        scale: Some(PxScale::from(50.0)),
    });

    canvas.draw(
        &new_target_text,
        DrawParam::from(Vec2::new(
            game.window_size.width as f32 / 7.0,
            game.window_size.height as f32 / 3.0,
        )),
    );

    canvas.draw(
        &press_space_text,
        DrawParam::from(Vec2::new(
            game.window_size.width as f32 / 7.0,
            game.window_size.height as f32 / 3.0 + 300.0,
        )),
    );
}

fn draw_intro_screen(_ctx: &Context, canvas: &mut Canvas, game: &GameState) {
    let description = format!("Catch bubbles in each round to make up the target.\nBubbles get faster with each round.\nTime bonus of 1 point for every {} seconds left.\nOvershooting costs a life!\n\nPress space to start...", constants::SECONDS_LEFT_PER_BONUS_POINT);

    let desc_text = Text::new(TextFragment {
        text: description,
        color: Some(Color::GREEN),
        font: Some("LiberationMono-Regular".into()),
        scale: Some(PxScale::from(30.0)),
    });

    canvas.draw(
        &desc_text,
        DrawParam::from(Vec2::new(
            game.window_size.width as f32 / 9.0,
            game.window_size.height as f32 / 3.0,
        )),
    );
}