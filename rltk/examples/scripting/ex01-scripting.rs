#![warn(clippy::pedantic)]

rltk::add_wasm_support!();

use bracket_script::prelude::*;
use bracket_script::driver::Driver;
use rltk::prelude::*;
use std::path::PathBuf;

struct State {
    pub script_path: PathBuf
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        driver::LuaDriver::exec_script(self.script_path.clone());
    }
}

// Every program needs a main() function!
fn main() -> RltkError {
    let context = RltkBuilder::simple80x50()
    .with_title("Hello RLTK World")
    .with_fps_cap(30.0)
    .build()?;
    
    // Now we create an empty state object.
    let mut gs: State = State {script_path : PathBuf::new()};
    gs.script_path.push("/home/pablo/repositorioGit/bracket-lib/rltk/examples/scripting/hello.lua");

    // Call into RLTK to run the main loop. This handles rendering, and calls back into State's tick
    // function every cycle. The box is needed to work around lifetime handling.
    rltk::main_loop(context, gs)
}
