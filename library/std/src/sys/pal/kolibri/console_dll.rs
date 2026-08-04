use core::ffi::c_char;
use core::sync::atomic::AtomicBool;

use super::dll;

pub struct Console {
    con_init: unsafe extern "stdcall" fn(u32, u32, u32, u32, *const c_char),
    con_write_string: unsafe extern "stdcall" fn(*const c_char, u32),
    con_exit: unsafe extern "stdcall" fn(bool),

    shown: AtomicBool
}

impl Console {
    pub fn init_from_lib() -> Option<Self> {
        let lib = dll::load_dll(c"/sys/lib/console.obj")?;

        let con_init = lib.iter().find(|en| en.name() == c"con_init")?;
        let con_write_string = lib.iter().find(|en| en.name() == c"con_write_string")?;
        let con_exit = lib.iter().find(|en| en.name() == c"con_exit")?;

        Some(Console {
            con_init: *con_init.data(),
            con_write_string: *con_write_string.data(),
            con_exit: *con_exit.data(),

            shown: AtomicBool::new(false)
        })
    }

    fn ensure_initialized(&self) {
        const DEFAULT: u32 = 0xffff_ffff;

        if !self.shown.load(crate::sync::atomic::Ordering::Acquire) {
            unsafe { (self.con_init)(DEFAULT, DEFAULT, DEFAULT, DEFAULT, c"Console application".as_ptr()) };

            self.shown.store(true, crate::sync::atomic::Ordering::Release);
        }
    }

    pub fn write(&self, data: &[u8]) {
        self.ensure_initialized();

        unsafe { (self.con_write_string)(data.as_ptr().cast(), data.len() as _) };
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        if self.shown.load(crate::sync::atomic::Ordering::Acquire) {
            unsafe { (self.con_exit)(false) };
        }
    }
}