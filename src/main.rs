use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    // provides a window into the currently running bracket-terminal—accessing information like mouse position and keyboard input, and sending commands to draw to the window.
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls(); // clears the game window every tick
        ctx.print(1, 1, "Hello World"); // just prints text to the screen @ location 1,1
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Crappy Dragon")
        .build()?; // finalize initialization into context. also pass errors to parent fx with ? if unable to initialize

    // implicitly run the main loop and return using context object and state. main and main_loop return a BError.
    main_loop(context, State{})
}
