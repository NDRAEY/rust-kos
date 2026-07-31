use crate::ptr;
use crate::sync::atomic::AtomicBool;
use crate::sync::atomic::AtomicPtr;
use crate::sync::atomic::Ordering;

#[repr(C)]
struct Tcb {
    self_ptr: *mut Tcb, // must be field 0
    slots: [AtomicPtr<u8>; MAX_KEYS],
}

pub type Key = usize;
const MAX_KEYS: usize = 128;

#[inline]
unsafe fn tcb() -> *mut Tcb {
    let base: *mut Tcb;

    unsafe { core::arch::asm!("mov {}, fs:0x0", out(reg) base, options(nostack, preserves_flags)) };

    base
}

#[inline]
unsafe fn tcb_mut() -> &'static mut Tcb {
    unsafe { &mut *tcb() }
}

pub unsafe fn get(key: Key) -> *mut u8 {
    unsafe { tcb_mut() }.slots[key].load(Ordering::Relaxed)
}

pub unsafe fn set(key: Key, value: *mut u8) {
    unsafe { tcb_mut() }.slots[key].store(value, Ordering::Relaxed);
}

struct Slot {
    used: AtomicBool,
    dtor: AtomicPtr<()>,
}

const EMPTY: Slot = Slot { used: AtomicBool::new(false), dtor: AtomicPtr::new(ptr::null_mut()) };
static REGISTRY: [Slot; MAX_KEYS] = [EMPTY; MAX_KEYS];

pub fn create(dtor: Option<unsafe extern "C" fn(*mut u8)>) -> Key {
    for (i, slot) in REGISTRY.iter().enumerate() {
        if slot.used.compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
            slot.dtor.store(dtor.map_or(ptr::null_mut(), |f| f as *mut ()), Ordering::Release);
            return i;
        }
    }
    rtabort!("out of TLS keys");
}

pub unsafe fn destroy(key: Key) {
    REGISTRY[key].used.store(false, Ordering::Release);
}
