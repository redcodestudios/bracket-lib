
extern crate cc;
use std::env;
use std::process::Command;

fn main() {
    if cfg!(target_os = "linux") {
        Command::new("make")
            .current_dir("lua")
            .status()
            .expect("failed to execute process");

        Command::new("make")
            .current_dir("cpython")
            .status()
            .expect("failed to execute process");
        
        cc::Build::new()
            .flag("-I")
            .flag("lua")
            // .include("../amethyst_lib/c_headers")
            .flag("-llua")
            // .object("../target/debug/libamethyst_lib.so")
            .file("c_drivers/lua_vm.c")
            .compile("lua_vm");
   
        cc::Build::new()
            .include("cpython")
            .include("cpython/Include")
            .flag("-lpython3.9")
            //.object("../target/debug/libscripting_api.so")
            .file("c_drivers/python_vm.c")
            .compile("python_vm");

        println!("cargo:rustc-flags=-l lua -L lua");
        println!("cargo:rustc-flags=-l python3.9 -L cpython");
    }
}
