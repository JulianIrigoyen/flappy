use bracket_lib::prelude::*;


/// BError allows to take advantag of "?" operator
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State{})
}

struct State {

}

impl GameState for State {
    //! &mut self allows the tick gfunction to acces and change your State instance
    //! ctx provides a window into the currently running bracket-terminal,
    //!    and provides functions to interact with game display. fn cls clears the window
    fn tick (&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, Bracket Terminal!");
    }
}