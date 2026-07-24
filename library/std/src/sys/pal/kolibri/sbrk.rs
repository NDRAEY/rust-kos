/// Rust interpretation of sbrk implementation from Newlib
/// 
/// Original source: https://git.kolibrios.org/KolibriOS/ports/src/branch/main/libraries/newlib/newlib/libc/sys/kolibrios/sbrk.c

use super::api::realloc;
use crate::sync::{Mutex, OnceLock};

struct Heap {
    start: *mut u8,
    brk: usize,
}

unsafe impl Sync for Heap {}
unsafe impl Send for Heap {}

impl Heap {
    pub fn new() -> Self {
        Self { start: core::ptr::null_mut(), brk: 0 }
    }
}

static HEAP: OnceLock<Mutex<Heap>> = OnceLock::new();

pub fn sbrk(increment: isize) -> Option<*mut u8> {
    let mut heap = HEAP.get_or_init(|| Mutex::new(Heap::new()));

    let mut bind = heap.lock().unwrap();

    // If increment is zero = return pointer to current brk.
    if increment == 0 {
        return Some(unsafe { bind.start.add(bind.brk) });
    }

    let new_brk;

    if increment < 0 {
        let decrement = (-increment) as usize;

        new_brk = bind.brk.checked_sub(decrement).unwrap_or(0);
    } else {
        new_brk = match bind.brk.checked_add(increment as usize) {
            Some(value) => value,
            None => {
                return None;
            }
        }
    }

    let new_start = realloc(bind.start, new_brk);

    let brk = unsafe { new_start.byte_add(bind.brk) };

    bind.start = new_start;
    bind.brk = new_brk;

    Some(brk)
}
