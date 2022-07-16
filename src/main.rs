use bracket_lib::prelude::*;
mod commons;
mod engine;
mod entity;

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_font("../resources/skeletox.png", 32, 32)
        .with_simple_console(
            crate::commons::SCREEN_WIDTH,
            crate::commons::SCREEN_HEIGHT,
            "../resources/skeletox.png",
        )
        .with_fancy_console(
            crate::commons::SCREEN_WIDTH,
            crate::commons::SCREEN_HEIGHT,
            "../resources/skeletox.png",
        )
        .with_title("Flappy Dragon Enhanced")
        .with_tile_dimensions(16, 16)
        .build()?;

    main_loop(context, crate::engine::State::new())
}
