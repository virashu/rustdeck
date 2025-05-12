use plugin_base::Plugin as TPlugin;
use std::ffi::{c_char, CStr, CString};
use std::pin::pin;

#[repr(C)]
pub struct Plugin {
    name: String,
    description: String,
    id: String,
}

impl Plugin {
    fn new() -> Self {
        Self {
            name: "Plugin".into(),
            description: "Some plugin".into(),
            id: "plugin_test".into(),
        }
    }
}

impl TPlugin for Plugin {
    fn get_name(&self) -> *mut c_char {
        let c_str = CString::new(self.name.clone()).unwrap();
        Box::new(c_str).into_raw()
    }
    fn get_description(&self) -> *mut c_char {
        todo!()
    }
    fn get_id(&self) -> *mut c_char {
        todo!()
    }
    fn get_variables(&self) -> *mut c_char {
        todo!()
    }
    fn get_actions(&self) -> *mut c_char {
        todo!()
    }
    #[allow(unused_variables, reason = "WIP")]
    fn execute_action(&self, id: *mut c_char) {
        todo!()
    }
    fn update(&mut self) {
        todo!()
    }
}

#[no_mangle]
pub extern "C" fn make() -> *mut dyn TPlugin {
    Box::into_raw(Box::new(Plugin::new()))
}
