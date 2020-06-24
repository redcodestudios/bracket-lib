use std::ffi::CString;
use std::os::raw::c_char;
use std::path::PathBuf;

use std::sync::{Arc, Mutex};

#[repr(C)]
#[derive(Clone)]
pub struct lua_State { private: [u8; 0] }


// Lua C API functions
extern {
    fn luaL_newstate() -> *mut lua_State;
    fn lua_close(l: *mut lua_State);
    fn luaL_openlibs(l: *mut lua_State);
}

// C driver functions
extern {
    fn call_python(path: *const c_char);
    fn call_lua(path: *const c_char);
    fn call_lua_return(path: *const c_char) -> *const c_char;
    fn call_lua_bytes(l: *mut lua_State, source: *const u8, size: usize);
}

#[derive(Clone)]
pub struct LuaVM {
    state: Arc<Mutex<*mut lua_State>>
}

impl LuaVM {
    fn clean_state(&mut self) {
        unsafe {
            let mut s = luaL_newstate();
            luaL_openlibs(s);
            let mut old_state = self.state.lock().unwrap();
            lua_close(*old_state);
            *old_state = s;
        }
    }
}


pub trait Driver {
    fn new() -> Self;
    fn exec_script(path: PathBuf) -> Result<(), ()>;
    fn exec_script_return(path: PathBuf) -> *const c_char;
    fn exec_bytes(self, source: Vec<u8>);
}

pub struct PythonDriver;
impl Driver for PythonDriver {
    fn new() -> Self {
        Self{}
    }

    fn exec_bytes(mut self, source: Vec<u8>){}
    
    fn exec_script(path: PathBuf) -> Result<(), ()>{
        unsafe{
            let script_path = String::from(path.to_str().unwrap());
            //let b = Box::new(transform);
            //let transform_ptr = Box::into_raw(b);
            
            call_python(
                CString::new(script_path).expect("CString::new failed").as_ptr()
            );
        }
        Ok(())
    }

    fn exec_script_return(path: PathBuf) -> *const c_char {
        return CString::new("aa").expect("Fail").as_ptr();
    }
}

unsafe impl std::marker::Send for LuaVM{}
unsafe impl std::marker::Sync for LuaVM{}
impl Driver for LuaVM {
    fn exec_bytes(mut self, source: Vec<u8>) {
         unsafe {
            &self.clean_state();
            let s = *Arc::try_unwrap(self.state).unwrap_err().lock().unwrap();
            call_lua_bytes(s, source.as_ptr(), source.len());
        }
    }
    fn new() -> Self {
        unsafe {
            let s = luaL_newstate();
            luaL_openlibs(s);
            Self { state: Arc::new(Mutex::new(s)) }
        }
    }
   

    fn exec_script(path: PathBuf) -> Result<(), ()> {
        unsafe{
            let script_path = String::from(path.to_str().unwrap());
            //let b = Box::new(transform);
            //let transform_ptr = Box::into_raw(b);
            
            call_lua(
                CString::new(script_path).expect("CString::new failed").as_ptr()
            );
        }
        Ok(())
    }

    fn exec_script_return(path: PathBuf) -> *const c_char { 
        let mut r: *const c_char;
        unsafe {
            let script_path = String::from(path.to_str().unwrap());
             r = call_lua_return(
                CString::new(script_path).expect("CString::new failed").as_ptr()
            );
        }
        r
    }
}

