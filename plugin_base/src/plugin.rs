use std::ffi::c_char;

pub trait Plugin {
    fn get_name(&self) -> *mut c_char;
    fn get_description(&self) -> *mut c_char;
    fn get_id(&self) -> *mut c_char;

    fn get_variables(&self) -> *mut c_char;
    fn get_actions(&self) -> *mut c_char;

    fn execute_action(&self, id: *mut c_char);

    fn update(&mut self);
}
