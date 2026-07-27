use crate::sync::atomic::Atomic;
use crate::time::Duration;


pub type Primitive = u32;
pub type FutexInner = Atomic<Primitive>;

pub struct Futex {
    inner: Box<FutexInner>,
    fd: usize
}

pub type SmallFutex = Futex;
pub type SmallPrimitive = Primitive;

use super::syscall::*;

impl Futex {
    pub fn new(value: Primitive) -> Option<Self> {
        let inner = Box::new(FutexInner::new(value));

        let result = unsafe { syscall3(77, 0, (&*inner as *const FutexInner).addr()) };

        if result == 0 {
            None
        } else {
            Some(Futex {
                inner,
                fd: result
            })
        }
    }
}

impl AsRef<FutexInner> for Futex {
    fn as_ref(&self) -> &FutexInner {
        &self.inner
    }
}

impl Drop for Futex {
    fn drop(&mut self) {
        unsafe { syscall3(77, 1, self.fd) };
    }
}

/// Waits for a `futex_wake` operation to wake us.
///
/// Returns directly if the futex doesn't hold the expected value.
///
/// Returns false on timeout, and true in all other cases.
pub fn futex_wait(futex: &Futex, expected: u32, timeout: Option<Duration>) -> bool {
    let result = unsafe { syscall5(77, 2, futex.fd, expected as _, timeout.map(|x| x.as_millis() as usize / 10).unwrap_or(0)) };

    let result = result as i32;

    if result == -1 {
        false
    } else {
        true
    }
}

/// Wakes up one thread that's blocked on `futex_wait` on this futex.
///
/// Returns true if this actually woke up such a thread,
/// or false if no thread was waiting on this futex.
///
/// On some platforms, this always returns false.
pub fn futex_wake(futex: &Futex) -> bool {
    let result = unsafe { syscall4(77, 3, futex.fd, 1) };

    result > 0
}

pub fn futex_wake_all(futex: &Futex) {
    unsafe {
        syscall4(77, 3, futex.fd, usize::MAX);
    }
}
