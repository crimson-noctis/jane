#![allow(clippy::missing_safety_doc)]
use crate::repl::Repl;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn repl_new() -> *mut Repl {
    Box::into_raw(Box::new(Repl::new()))
}

// Safety: idk
#[unsafe(no_mangle)]
pub unsafe extern "C" fn repl_run_examples(repl: *mut Repl) {
    let repl = unsafe { &mut *repl };
    repl.run_examples();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn repl_command(repl: *mut Repl, cmd: *const c_char) -> *mut c_char {
    let repl = unsafe { &mut *repl };
    let cmd = unsafe { CStr::from_ptr(cmd) }.to_str().unwrap_or("");

    let result = match repl.process_command(cmd) {
        Ok(output) => output,
        Err(e) => format!("ERROR: {}", e),
    };

    CString::new(result).unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn repl_should_quit(repl: *mut Repl) -> bool {
    let repl = unsafe { &mut *repl };
    repl.should_quit()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn repl_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            drop(CString::from_raw(s));
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn repl_free(repl: *mut Repl) {
    if !repl.is_null() {
        unsafe {
            drop(Box::from_raw(repl));
        }
    }
}
