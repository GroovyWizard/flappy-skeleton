use bracket_lib::prelude::*;
use engine::State;
mod engine;

fn main() -> BError {
    let context: BTerm = BTermBuilder::simple80x50().with_title("Flappy ").build()?;
    main_loop(context, State::new())
}
