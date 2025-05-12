use std::ffi::{c_char, c_void};

pub mod interface {
    pub const NAME_IDENT: &[u8] = b"get_name";
    pub const DESCRIPTION_IDENT: &[u8] = b"get_description";
    pub const ID_IDENT: &[u8] = b"get_id";
    pub const PLUGIN_IDENT: &[u8] = b"PLUGIN";
}

pub trait PluginTrait {
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
    fn get_id(&self) -> String;

    fn get_variables(&self) -> String;
    fn get_actions(&self) -> String;

    fn execute_action(&self, id: String);

    fn update(&mut self);
}

pub trait RsPluginTrait {
    fn get_name(&self) -> *mut c_char;
    fn get_description(&self) -> *mut c_char;
    fn get_id(&self) -> *mut c_char;

    fn get_variables(&self) -> *mut c_char;
    fn get_actions(&self) -> *mut c_char;

    fn execute_action(&self, id: *mut c_char);

    fn update(&mut self);
}

#[rustfmt::skip]
#[repr(C)]
pub struct CPlugin {
    pub new             : unsafe extern "C" fn() -> *mut c_void,

    // pub get_variables   : unsafe extern "C" fn(state: *mut c_void) -> *mut c_char,
    // pub get_actions     : unsafe extern "C" fn(state: *mut c_void) -> *mut c_char,

    pub execute_action  : unsafe extern "C" fn(state: *mut c_void, id: *mut c_char),
    pub update          : unsafe extern "C" fn(state: *mut c_void),
}

#[macro_export]
macro_rules! define_plugin {
    (
        name: $name:literal,
        description: $description:literal,
        id: $id:literal,
        data: $data:expr
    ) => {
        const __NAME: &str = concat!($name, "\0");
        const __DESCRIPTION: &str = concat!($description, "\0");
        const __ID: &str = concat!($id, "\0");

        #[no_mangle]
        pub extern "C" fn get_name() -> *const ::std::os::raw::c_char {
            __NAME.as_ptr() as _
        }

        #[no_mangle]
        pub extern "C" fn get_description() -> *const ::std::os::raw::c_char {
            __DESCRIPTION.as_ptr() as _
        }

        #[no_mangle]
        pub extern "C" fn get_id() -> *const ::std::os::raw::c_char {
            __ID.as_ptr() as _
        }

        #[no_mangle]
        static PLUGIN: $crate::CPlugin = $data;
    };
}
