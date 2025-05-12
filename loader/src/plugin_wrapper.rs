use libloading::{Library, Symbol};
use std::{
    error::Error,
    ffi::{c_char, CStr, OsStr},
    fmt::Debug,
};

use plugin_base::Plugin;

type DynPlugin = Box<dyn Plugin>;
type PluginBuilderFn = unsafe extern "C" fn() -> *mut dyn Plugin;

fn read_drop_pointer(ptr: *mut c_char) -> String {
    let c_str: &CStr = unsafe { CStr::from_ptr(ptr) };
    let str_slice: &str = c_str.to_str().unwrap();
    let string = str_slice.to_owned();

    unsafe {
        std::ptr::drop_in_place(ptr);
    }

    string
}

pub struct PluginWrapper {
    #[allow(dead_code)] // Needed because unsafe
    lib: Library,
    plugin: Box<dyn Plugin>,
}

impl PluginWrapper {
    pub fn try_load<P: AsRef<OsStr> + Debug>(path: P) -> Result<Self, Box<dyn Error>> {
        unsafe {
            let lib = Library::new(path)?;

            let build_fn: Symbol<PluginBuilderFn> = lib.get(b"make")?;
            let plugin: DynPlugin = Box::from_raw(build_fn());

            Ok(Self { lib, plugin })
        }
    }

    pub fn get_name(&self) -> String {
        read_drop_pointer(self.plugin.get_name())
    }

    // fn get_description(&self) -> String {}
    // fn get_id(&self) -> String {}

    // fn get_variables(&self) -> Vec<String> {}
    // fn get_actions(&self) -> Vec<String> {}

    // fn execute_action(&self, id: String) {}

    // fn update(&mut self) {}
}

impl Drop for PluginWrapper {
    fn drop(&mut self) {}
}
