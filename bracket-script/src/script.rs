use std::path::{PathBuf, Path};
use std::fs;
use std::io::Read;

use legion::prelude::Resources;

#[derive(Clone)]
pub struct Script {
    pub path: PathBuf,
    pub bytes: Vec<u8>
}

impl Script {
    pub fn new(path_str: &str) -> Self {
        let mut f = fs::File::open(path_str).unwrap();
        let mut bytes_buffer = Vec::new();

        f.read_to_end(&mut bytes_buffer);

        Self {
            path: PathBuf::from(path_str),
            bytes: bytes_buffer 
        }
    }

    
    pub fn load_scripts_from_folder(mut resources: &mut Resources, mut scripts: &mut Vec<Script>, dir_path: &str)-> Vec<Self>{
        if Path::new(dir_path).is_dir() {
            for e in fs::read_dir(dir_path).unwrap() {
                let entry = e.unwrap();
               if !entry.path().is_dir() {
                   let script = Script::new(entry.path().to_str().unwrap());
                   (*resources).insert(script.clone());
                   (*scripts).push(script.clone());
                }else{
                    Self::load_scripts_from_folder(&mut resources, &mut scripts, entry.path().to_str().unwrap());
               }
            }
        }
        return scripts.to_vec();
    }
    pub fn load_multiple(mut resources: Resources, dir_path: &str) -> Vec<Self> {
        let mut scripts: Vec<Script> = Vec::new();
        println!("Loading scripts...");
        scripts = Self::load_scripts_from_folder(&mut resources,&mut scripts, dir_path);

        if(scripts.is_empty()){
            println!("No scripts found in folder!");
        }else{
            for s in &scripts {
                println!("\tLoaded: {}", s.path.display());
            }
        }

        scripts
    }
    pub fn to_utf8(self) -> String {
        String::from_utf8(self.bytes).unwrap()
    }
}
