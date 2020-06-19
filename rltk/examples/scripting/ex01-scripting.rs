#![warn(clippy::pedantic)]

rltk::add_wasm_support!();

use bracket_script::prelude::*;
use bracket_script::driver::Driver;
use rltk::prelude::*;
use std::path::PathBuf;
use std::slice;

struct State {
    pub script_path: PathBuf
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        let str = driver::LuaDriver::exec_script_return(self.script_path.clone());
        let col1 = RGB::named(rltk::CYAN);
        let percent: f32 = 143 as f32 / 50.0;
        let fg = col1.lerp(col1, percent);
        ctx.cls();

        let str_bytes;

        unsafe {
           str_bytes =  slice::from_raw_parts(str as *const u8, 8).to_vec();
        }

        ctx.print_color(
            1,
            1,
            fg,
            RGB::named(rltk::BLACK),
            String::from_utf8(str_bytes).unwrap(),
        );
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
    gs.script_path.push("/home/shammyz/Documents/repos/bracket-lib/rltk/examples/scripting/hello.lua");

    // Call into RLTK to run the main loop. This handles rendering, and calls back into State's tick
    // function every cycle. The box is needed to work around lifetime handling.
    rltk::main_loop(context, gs)
}
