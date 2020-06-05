use std::path::PathBuf;

mod driver;

use driver::{Driver, LuaDriver};

pub fn main() {
    let mut script_path = PathBuf::new();
    script_path.push("/home/shammyz/Documents/test.lua");
    LuaDriver::exec_script(script_path);
}
