use tetra::graphics::{self, Color};
use tetra::{Context, ContextBuilder, State};

fn main() -> tetra::Result {
    ContextBuilder::new("SIGMA", 640, 480)
        .quit_on_escape(true)
        .build()?
        .run(|_| Ok(GameState {}))
}

struct GameState {}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        Ok(())
    }
}