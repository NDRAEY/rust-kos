#![deny(unsafe_op_in_unsafe_fn)]
#![allow(unused)]

mod common;
pub use common::*;

pub mod api;
pub mod dll;

mod syscall;

use crate::ffi::CStr;

// const CMDLINE_SIZE: usize = 256;
// const PATH_SIZE: usize = 1024;

unsafe extern "C" {
    #[allow(improper_ctypes)]
    static ___kosapp_cmdline: ();
    #[allow(improper_ctypes)]
    static ___kosapp_name: ();
}

fn application_path_raw() -> *const crate::ffi::c_char {
    unsafe { (&___kosapp_name as *const ()).cast() }
}

fn command_line_raw() -> *const crate::ffi::c_char {
    unsafe { (&___kosapp_cmdline as *const ()).cast() }
}

/// Returns application path
/// Note: Application path that is being stored in app has following encoding:
///       u8; str.
///       ^   ^-- The path itself.
///       `-- Encoding byte: 01 = CP866; 02 = UTF16-LE; 03 = UTF-8
/// For more info: https://wiki.kolibrios.org/wiki/SysFn70/ru (in Russian!)
pub fn application_path() -> &'static CStr {
    unsafe { CStr::from_ptr(application_path_raw().byte_add(1)) }
}

/// Returns a string containing raw command line arguments (without split and without argv[0] which is an application path, see function above).
pub fn command_line() -> &'static CStr {
    unsafe { CStr::from_ptr(command_line_raw()) }
}