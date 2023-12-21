use exploring_evolutionary_programming_ggez::MainState;
use ggez::{
    conf::{WindowMode, WindowSetup},
    event, ContextBuilder,
};

fn main() {
    let width = 1200.0;
    let height = 600.0;

    let (context, event_loop) = ContextBuilder::new("exploring_evolutionary_coding", "Brookzerker")
        .window_mode(WindowMode::default().dimensions(width, height))
        .build()
        .unwrap();
    let state = MainState::new(width, height);

    event::run(context, event_loop, state);
}
