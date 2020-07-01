#![warn(clippy::pedantic)]

rltk::add_wasm_support!();

use legion::prelude::*;

use bracket_script::prelude::{script::*, driver::*, script_system};
// use bracket_script::driver::Driver;

use rltk::prelude::*;
use std::path::PathBuf;
use std::slice;
use std::env;


struct State {
    // pub scripts: Vec<Script>,
    pub lua_vm: LuaVM,
    pub systems: Vec<Schedule>,
    pub resources: Resources,
    pub world: World,
}
impl State{
    fn run_systems(&mut self) {
        for systems in self.systems.iter_mut() {
            systems.execute(&mut self.world, &mut self.resources);
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        
        // for s in &self.scripts {
        //     self.lua_vm.clone().exec_bytes(s.bytes.clone());
        // }
        self.run_systems();
        // let str = vec![1,1];
        // let str = driver::LuaVM::exec_bytes(self.script_path.clone());
        let col1 = RGB::named(rltk::CYAN);
        let percent: f32 = 143 as f32 / 50.0;
        let fg = col1.lerp(col1, percent);
        ctx.cls();

        // let str_bytes;

        // unsafe {
           // str_bytes =  slice::from_raw_parts(str as *const u8, 8).to_vec();
        // }

        ctx.print_color(
            1,
            1,
            fg,
            RGB::named(rltk::BLACK),
            "Hello World!"
            // String::from_utf8(str_bytes).unwrap(),
        );

        ctx.print_color(
            40,
            1,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            &format!("FPS: {}", ctx.fps),
        );

        ctx.print_color(
            40,
            2,
            RGB::named(rltk::CYAN),
            RGB::named(rltk::BLACK),
            &format!("Frame Time: {} ms", ctx.frame_time_ms),
        );
    }
}

// Every program needs a main() function!
fn main() -> RltkError {
    let context = RltkBuilder::simple80x50()
    .with_title("Hello RLTK World")
    .with_fps_cap(100.0)
    .build()?;

    let universe = Universe::new();
    let mut world = universe.create_world();
    let mut resources = Resources::default();
    
    let cur_path = env::current_dir()?;
    let lua_scripts = cur_path.join("examples/scripting/lua");

    let mut scripts = Script::load_all(&lua_scripts);

    let systems = vec![
        Schedule::builder()
            .add_system(script_system::DriverHolder::<LuaVM>::build(&mut scripts))
            .build()
    ];

    resources.insert(scripts);

    // Now we create an empty state object.
    // let mut script_path = cur_path.join("examples/scripting/hello.lua");
    // println!("{}",script_path.display());
    let mut gs: State = State { 
        lua_vm: LuaVM::new(),
        systems: systems,
        resources: resources,
        world: world,
    };
    // Call into RLTK to run the main loop. This handles rendering, and calls back into State's tick
    // function every cycle. The box is needed to work around lifetime handling.
    rltk::main_loop(context, gs)
}
