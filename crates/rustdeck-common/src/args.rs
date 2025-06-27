use crate::{proto::Arg, util::ptr_to_str};

/// (unsafe) A struct designed to provide args for actions
///
/// # Example
/// ```rust
/// use rustdeck_common::Args;
///
/// fn run_action(state: &(), id: &str, args: &Args) {
///     if id == "some_action" {
///         let arg1: i32 = args.get(0).int();
///         let arg2: &str = args.get(1).string();
///     }
/// }
/// ```
///
/// # Safety
/// Requested args **must** be described in plugin's actions declaration,
/// then amount, and types of args can be guaranteed by Rustdeck.
pub struct Args {
    inner: *const Arg,
}

impl Args {
    pub fn from(arg_ptr: *const Arg) -> Self {
        Self { inner: arg_ptr }
    }

    pub fn get(&self, n: isize) -> ArgsInst {
        ArgsInst::from(unsafe { self.inner.offset(n) })
    }
}

pub struct ArgsInst {
    // With offset
    inner: *const Arg,
}

impl ArgsInst {
    pub fn from(arg_ptr: *const Arg) -> Self {
        Self { inner: arg_ptr }
    }

    pub fn bool(&self) -> bool {
        unsafe { *self.inner.as_ref().unwrap().b.as_ref().unwrap() }
    }

    pub fn int(&self) -> i32 {
        unsafe { *self.inner.as_ref().unwrap().i.as_ref().unwrap() }
    }

    pub fn float(&self) -> f32 {
        unsafe { *self.inner.as_ref().unwrap().f.as_ref().unwrap() }
    }

    pub fn string(&self) -> &str {
        unsafe { ptr_to_str(self.inner.as_ref().unwrap().c) }
    }
}
