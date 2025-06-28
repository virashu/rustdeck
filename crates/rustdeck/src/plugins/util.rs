// use std::ffi::{CStr, CString, c_char};

// pub unsafe fn read_drop_pointer(ptr: *mut c_char) -> String {
//     if ptr.is_null() {
//         return String::new();
//     }

//     unsafe { CString::from_raw(ptr) }.into_string().unwrap()
// }

// pub unsafe fn read_drop_pointer(ptr: *mut c_char) -> String {
//     if ptr.is_null() {
//         return String::new();
//     }

//     let c_str: &CStr = unsafe { CStr::from_ptr(ptr) };
//     let str_slice: &str = c_str.to_str().unwrap();
//     let string = str_slice.to_owned();

//     unsafe { std::ptr::drop_in_place(ptr) };

//     string
// }
