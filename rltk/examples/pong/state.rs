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
    pub fn update_key(self, ctx: Rltk){
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk){
        // self.update_key(ctx.clone());
        {
            let mut key = self.resources.get_mut::<Option<VirtualKeyCode>>().unwrap();
            match ctx.key {
                None => *key = None::<VirtualKeyCode>,
                Some(x) => *key = Some(x)
            }
        }
        
        for s in self.systems.iter_mut() {
            s.execute(&mut self.world, &mut self.resources);
        }  
        
    }
}
