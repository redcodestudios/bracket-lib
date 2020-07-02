use legion::prelude::*;

// mod driver;
use std::marker::PhantomData;
use crate::{script::*, driver::*};

pub struct DriverHolder <D:Driver>{phantom: PhantomData<D>}

impl <D: std::clone::Clone + 'static + Driver + std::marker::Send + std::marker::Sync> DriverHolder <D>{
    pub fn build(unloaded_scripts: &mut Vec<Script>) -> Box<(dyn Schedulable + 'static)> {
        let driver_instance = D::new();
        for s in unloaded_scripts{
            match s.state {
                ScriptState::Unloaded => {
                    driver_instance.clone().exec_script_setup(s.bytes.clone());
                    s.state = ScriptState::Loaded;                   
                },
                _ => print!("Trying to load an already loaded script!"),
            }
        }

        SystemBuilder::new("scripting")
            .read_resource::<Vec<Script>>()
            .build(
                move |_commands, mut world, scripts, query| {
                    for s in &**scripts{
                        match s.state {
                            // TODO: exec_script_update is a empty function for now
                            ScriptState::Loaded => driver_instance.clone().exec_script_update(s.bytes.clone()),
                            _ => print!("Trying to run an unloaded script!"),
                        }
                         
                    }
                })
    }
}

// driver is only a string for now

