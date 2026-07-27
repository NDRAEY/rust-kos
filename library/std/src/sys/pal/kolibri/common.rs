use crate::io as std_io;

use crate::fmt::Write;

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
    writeln!(
        super::api::debugboard(),
        "Application path: {:?}",
        super::application_path()
    )
    .ok();

    writeln!(super::api::debugboard(), "Command line: {:?}", super::command_line()).ok();
}

// SAFETY: must be called only once during runtime cleanup.
// NOTE: this is not guaranteed to run, for example when the program aborts.
pub unsafe fn cleanup() {
    writeln!(super::api::debugboard(), "TOOD: Do cleanup, close sockets").ok();

}

pub fn unsupported<T>() -> std_io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> std_io::Error {
    std_io::Error::UNSUPPORTED_PLATFORM
}

pub fn abort_internal() -> ! {
    core::intrinsics::abort();
}
