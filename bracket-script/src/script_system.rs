use legion::prelude::*;

// mod driver;
use std::marker::PhantomData;
use crate::{script::*, driver::*};

pub struct DriverHolder <D:Driver>{phantom: PhantomData<D>}

impl <D: std::clone::Clone + 'static + Driver + std::marker::Send + std::marker::Sync> DriverHolder <D>{
    pub fn build() -> Box<(dyn Schedulable + 'static)> {
        let driver_instance = D::new();
        SystemBuilder::new("scripting")
            .read_resource::<Vec<Script>>()
            .build(
                move |_commands, mut world, scripts, query| {
                    for s in &**scripts{
                        driver_instance.clone().exec_bytes(s.bytes.clone()); 
                    }
                })
    }
}

// driver is only a string for now

