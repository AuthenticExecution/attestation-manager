use std::collections::HashMap;
use std::sync::Mutex;
use anyhow::Result;

use crate::module::Module;
use crate::error::Error;

lazy_static! {
    pub static ref MODULES: Mutex<HashMap<String, Module>> = {
        Mutex::new(HashMap::new())
    };
}

pub fn add_module(name : String, module : Module) -> Result<()> {
    let mut map = MODULES.lock().unwrap();

    map.insert(name, module);
    Ok(())
}

// Uncomment if needed
/* 
pub fn get_module_copy(name : &str) -> Option<Module> {
    let map = MODULES.lock().unwrap();
    map.get(name).and_then(|m| Some(m.clone()))
}
*/

pub fn get_module_key(name : &str) -> Option<Vec<u8>> {
    let map = MODULES.lock().unwrap();

    map.get(name).map(|m| m.key.clone())
}

pub fn clear_map() {
    let mut map = MODULES.lock().unwrap();
    map.clear();
}
