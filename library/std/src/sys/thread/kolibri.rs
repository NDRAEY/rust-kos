use crate::ffi::CStr;
use crate::io;
use crate::num::NonZero;
use crate::thread::ThreadInit;
use crate::time::Duration;

use alloc::alloc::{alloc, Layout};
use crate::sys::pal::api;

use core::fmt::Write;

extern "C" fn __rust_kos_thread_start(raw_threadinit: *mut ThreadInit) {
    writeln!(api::debugboard(), "Entry point: {:?}", raw_threadinit).unwrap();

    unsafe { 
        let init = Box::from_raw(raw_threadinit);

        // Uncomment that when I get those thread_local! to work
        // let rust_start = init.init();

        // Sly kludge hehehehehe
        let rust_start = init.rust_start;

        rust_start();
    }

    api::exit(0)
}


// Silence dead code warnings for the otherwise unused ThreadInit::init() call.
#[expect(dead_code)]
fn dummy_init_call(init: Box<ThreadInit>) {
    drop(init.init());
}

#[expect(dead_code)]
pub struct Thread {
    id: usize,
    stack_area: *mut usize,
    stack_size: usize
}

unsafe impl Sync for Thread {}
unsafe impl Send for Thread {}

pub const DEFAULT_MIN_STACK_SIZE: usize = 64 * 1024;

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new(stack_size: usize, init: Box<ThreadInit>) -> io::Result<Thread> {
        let stack_area = unsafe { alloc(Layout::from_size_align_unchecked(stack_size, 16)) }.cast::<usize>();
        let mut stack_top = stack_area.byte_add(stack_size);

        writeln!(api::debugboard(), "Stack area: {:?} (size is: {stack_size})", stack_area).unwrap();
        
        let init = Box::into_raw(init);

        // Skip these 4 bytes, they are reserved.
        stack_top = stack_top.byte_sub(core::mem::size_of::<usize>());
        stack_top.write(init.addr());
        // Write address of entry point info to the new thread stack.
        stack_top = stack_top.byte_sub(core::mem::size_of::<usize>());
        stack_top.write(init.addr());

        let result = api::spawn_thread(__rust_kos_thread_start as *const () as *const _, stack_top);

        writeln!(api::debugboard(), "Spawned: {:?}", result).unwrap();

        if (result as i32) == -1 {
            // I don't know what kind of error return on "Too many threads" case.
            return Err(io::ErrorKind::Other.into());
            // return Err(io::ErrorKind::StorageFull);
        }

        Ok(Thread {
            id: result,
            stack_area,
            stack_size
        })
    }

    pub fn join(self) {
        // TODO: Wait for thread and clean its resources
        unimplemented!("Thread::join() is not implemented!")
    }
}

pub fn available_parallelism() -> io::Result<NonZero<usize>> {
    // TODO: KolibriOS doesn't support SMP now. Edit this function when the miracle happens.
    Ok(NonZero::new(1).unwrap())
}

pub fn current_os_id() -> Option<u64> {
    None
}

pub fn yield_now() {
    crate::sys::pal::api::yield_now();
    // do nothing
}

pub fn set_name(_name: &CStr) {
    // nope
}

pub fn sleep(dur: Duration) {
    crate::sys::pal::api::sleep_this_thread(dur.as_nanos());
}
