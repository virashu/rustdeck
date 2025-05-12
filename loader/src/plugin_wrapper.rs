use libloading::{Library, Symbol};
use std::{
    error::Error,
    ffi::{c_char, c_void, CStr, OsStr},
    fmt::Debug,
};

use common::{interface, CPlugin, PluginTrait};

use crate::error::report_libloading_error;

unsafe fn get_str<'a>(library: &'a Library, ident: &[u8]) -> Result<&'a str, Box<dyn std::error::Error>> {
    // First, the string exported by the plugin is read. For FFI-safety and
    // thread-safety, this must be a function that returns `*const c_char`.
    let name_fn = library.get::<extern "C" fn() -> *const c_char>(ident)?;
    let name: *const c_char = name_fn();

    // Unfortunately there is no way to make sure this part is safe. We have
    // to assume the address exported by the plugin is valid. Otherwise,
    // this part may cause an abort.
    let name = CStr::from_ptr(name);

    // Finally, the string is converted to UTF-8 and returned
    Ok(name.to_str()?)
}

pub struct RsPluginWrapper {
    name: String,
    description: String,
    id: String,

    plugin_data: CPlugin,
    state: *const c_void,

    #[allow(dead_code, reason = "`plugin` depends on `lib`")]
    lib: Library,
}

impl RsPluginWrapper {
    pub fn try_load<P: AsRef<OsStr> + Debug>(path: P) -> Result<Self, Box<dyn Error>> {
        unsafe {
            let lib = Library::new(path).inspect_err(report_libloading_error)?;

            let name = get_str(&lib, interface::NAME_IDENT)?.to_owned();
            let description = get_str(&lib, interface::DESCRIPTION_IDENT)?.to_owned();
            let id = get_str(&lib, interface::ID_IDENT)?.to_owned();

            let plugin_data = lib
                .get::<Symbol<*const CPlugin>>(interface::PLUGIN_IDENT).unwrap()
                .read();

            let state = (plugin_data.new)();

            Ok(Self {
                name,
                id,
                description,
                plugin_data,
                state,
                lib,
            })
        }
    }
}

impl PluginTrait for RsPluginWrapper {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_actions(&self) -> String {
        todo!()
    }
    fn get_description(&self) -> String {
        todo!()
    }
    fn get_id(&self) -> String {
        todo!()
    }
    fn get_variables(&self) -> String {
        todo!()
    }
    fn update(&mut self) {
        todo!()
    }
    fn execute_action(&self, id: String) {
        todo!()
    }
}
