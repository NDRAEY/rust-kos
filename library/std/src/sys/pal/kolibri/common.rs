use crate::io as std_io;

unsafe extern "C" {
    fn main();
}

#[unsafe(no_mangle)]
#[allow(unused)]
pub extern "C" fn _start() -> ! {
    unsafe {
        main();
    };

    super::api::exit(0)
}

// SAFETY: must be called only once during runtime initialization.
// NOTE: this is not guaranteed to run, for example when Rust code is called externally.
pub unsafe fn init(_argc: isize, _argv: *const *const u8, _sigpipe: u8) {
    super::api::debugboard_write_str("WARNING/TODO: Initializer function called. Implement it to use `std::env::args()`\n");
}

// SAFETY: must be called only once during runtime cleanup.
// NOTE: this is not guaranteed to run, for example when the program aborts.
pub unsafe fn cleanup() {}

pub fn unsupported<T>() -> std_io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> std_io::Error {
    std_io::Error::UNSUPPORTED_PLATFORM
}

pub fn abort_internal() -> ! {
    core::intrinsics::abort();
}
