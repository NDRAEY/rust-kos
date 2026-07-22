use super::syscall::*;

#[inline]
pub fn exit(_code: u32) -> ! {
    unsafe {
        syscall1(0xffff_ffff)
    };

    loop {}
}


#[inline]
pub fn init_heap() {
    unsafe { syscall2(68, 11) };
}

#[inline]
pub fn alloc(size: usize) -> *const u8 {
    unsafe { crate::ptr::with_exposed_provenance(syscall3(68, 12, size)) }
}

#[inline]
pub fn free(ptr: *const u8) {
    unsafe {
        syscall3(68, 13, ptr.addr() as _);
    }
}


#[inline]
pub fn debugboard_write(byte: u8) {
    unsafe { syscall3(63, 1, byte as _) };
}

#[inline]
pub fn debugboard_write_bulk(data: &[u8]) {
    for i in data {
        debugboard_write(*i);
    }
}

#[inline]
pub fn debugboard_write_str(data: &str) {
    debugboard_write_bulk(data.as_bytes());
}
