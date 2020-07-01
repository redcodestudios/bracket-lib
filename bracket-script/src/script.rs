use std::path::{PathBuf, Path};
use std::fs;
use std::io::Read;

#[derive(Clone)]
pub enum ScriptState {Loaded, Unloaded}

#[derive(Clone)]
pub struct Script {
    pub path: PathBuf,
    pub bytes: Vec<u8>,
    pub state: ScriptState,
}

impl<'a> Script {
    pub fn new(path_str: &str) -> Self {
        let mut f = fs::File::open(path_str).unwrap();
        let mut bytes_buffer = Vec::new();

        f.read_to_end(&mut bytes_buffer);

        Self {
            path: PathBuf::from(path_str),
            bytes: bytes_buffer,
            state: ScriptState::Unloaded
        }
    }

   /// Load all scripts inside a directory and his children recursively.
   pub fn load_all(dir_path: &Path) -> Vec<Script> {
       let entries = fs::read_dir(dir_path).unwrap();
       let mut scripts: Vec<Script> = Vec::new();

       for e in entries {
           let entry = e.unwrap();
           if entry.path().is_file() {
               scripts.push(Script::new(entry.path().to_str().unwrap()));
           } else {
               scripts.extend(Script::load_all(&entry.path()))
           }
       }

       scripts
   } 
}
