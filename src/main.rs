use tetra::graphics::{self, Color};
use tetra::{Context, ContextBuilder, State};

const DEFAULT_WINDOW_WIDTH: f32 = 1280.0;
const DEFAULT_WINDOW_HEIGHT: f32 = 720.0;

const DEFAULT_SCREEN_BACKGROUND: Color = Color::rgb(0.208, 0.631, 0.035);

fn main() -> tetra::Result {
    ContextBuilder::new("SIGMA", DEFAULT_WINDOW_WIDTH as i32, DEFAULT_WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(|_| Ok(GameState {}))
}

struct GameState {}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, DEFAULT_SCREEN_BACKGROUND);

        Ok(())
    }
}