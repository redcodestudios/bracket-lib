use legion::prelude::*;

mod driver;

use driver::*;

// driver is only a string for now
pub fn build(driver: Driver) -> Box<(dyn Schedulable + 'static)> {
    SystemBuilder::new("scripting")
        .read_resource::<Script>()
        .build(
            |_commands, mut world, scripts| {
                for s in scripts {
                    driver::run(s) 
                }
            })
}
