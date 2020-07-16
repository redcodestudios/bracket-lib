use rltk::prelude::*;

use legion::prelude::*;

mod pad;
mod state;
mod component;

use pad::*;
use state::State; 
use component::*;


fn main() -> RltkError {
    let context: Rltk = RltkBuilder::simple80x50()
        .with_title("Pong Game")
        .with_fps_cap(100.0)
        .build()?;

    let universe = Universe::new();
    let mut world = universe.create_world();
    let mut resources = Resources::default();
    
    // let outro_recurso = String::from("xaubb");

    // let boxed_context = Box::new(context);

    // resources.insert(outro_recurso);
    //
    // let ref_context = legion::borrow::AtomicRefCell::new(context);
    let key_pressed: Option<VirtualKeyCode> = None;
    resources.insert(key_pressed);

    // teste(outro_recurso);
    
    world.insert(
        (Pad(Side::Left),),
        (0..1).map(|_|(Transform{x: 0, y: 0},)),
    );
    
    world.insert(
        (Pad(Side::Right),),
        (0..1).map(|_|(Transform{x: 0, y: 0},)),
    );

    let systems = vec![
        Schedule::builder()
            .add_system(pad::move_sys_build())
            .build()
    ];

    let mut gs = State::new(world, resources, systems);
    // let context = &*(gs.resources.get::<Rltk>().unwrap());

    rltk::main_loop(context, gs)
}
