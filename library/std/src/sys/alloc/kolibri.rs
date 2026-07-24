//! Global Allocator for KolibriOS.
//! FIXME: Make a real `sbrk` based allocator.

const PAGE_SIZE: usize = 4096;
static mut MAIN_SECTOR: usize = 0;

use crate::alloc::Layout;
use crate::sync::atomic::{AtomicBool, Ordering};

use crate::sys::pal::api;

static HEAP_INIT: AtomicBool = AtomicBool::new(false);

#[allow(implicit_provenance_casts)]
fn init() {
    if !HEAP_INIT.swap(true, Ordering::Relaxed) {
        unsafe {
            api::init_heap();

            MAIN_SECTOR = api::alloc(PAGE_SIZE).addr();

            core::ptr::write_bytes(
                MAIN_SECTOR as *mut u8,
                0,
                PAGE_SIZE,
            ); // make sector table start clean
        }
    }
}

use super::realloc_fallback;

#[allow(implicit_provenance_casts)]
pub unsafe fn alloc(layout: Layout) -> *mut u8 {
    init();

    // Ensure that there is enough memory for alignment and marker
    let block = api::alloc(layout.size() + layout.align() + 1);
    let aligned_block = (block.addr() + 1).next_multiple_of(layout.align()) as *mut u8;

    let delta = (aligned_block.addr() - block.addr()) as u8;

    unsafe { aligned_block.byte_sub(1).write(delta) };

    (aligned_block) as *mut u8
}

pub unsafe fn dealloc(ptr: *mut u8, _: Layout) {
    let delta = unsafe { ptr.byte_sub(1).read() };

    let real_block = unsafe { ptr.byte_sub(delta as _) };

    api::free(real_block);
}

pub unsafe fn realloc(ptr: *mut u8, old_layout: Layout, new_size: usize) -> *mut u8 {
    unsafe { realloc_fallback(ptr, old_layout, new_size) }
}
