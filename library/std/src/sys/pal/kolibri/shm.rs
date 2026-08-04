use super::syscall::*;

use crate::ffi::CStr;

use super::error::{SystemResult, Error};

pub const SHM_OPEN: usize = 0x00;
pub const SHM_OPEN_ALWAYS: usize = 0x04;
pub const SHM_CREATE: usize = 0x08;

pub const SHM_READ: usize = 0x00;
pub const SHM_WRITE: usize = 0x01;

pub fn shm_open<'a>(name: &'a CStr, size: usize, flags: usize) -> SystemResult<(*mut u8, usize)> {
    let (eax, _, _, edx) = unsafe { syscall5_all(68, 22, name.as_ptr().addr(), size, flags) };

    if eax == 0 {
        return Err(unsafe { core::mem::transmute(edx) });
    }

    Ok((core::ptr::null_mut::<u8>().with_addr(eax), edx))
}

pub fn shm_close<'a>(name: &'a CStr) {
    unsafe { syscall3(68, 23, name.as_ptr().addr()) };
}