use rltk::prelude::*;

use legion::prelude::*;

pub struct State {
    pub world: World,
    pub resources: Resources,
    pub systems: Vec<Schedule>,
}

impl State {
    pub fn new(w: World, r: Resources, s: Vec<Schedule>) -> State {
        State { world: w, resources: r, systems: s}
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk){
        for s in self.systems.iter_mut() {
            s.execute(&mut self.world, &mut self.resources);
        }  
    }
}
