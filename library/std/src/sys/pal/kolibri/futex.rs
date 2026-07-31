use crate::sync::atomic::Atomic;
use crate::time::Duration;

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

pub type Primitive = u32;
pub type FutexInner = Atomic<Primitive>;

pub struct KFutex {
    inner: Box<FutexInner>,
    fd: usize
}

use super::syscall::*;

impl KFutex {
    pub fn new(value: Primitive) -> Self {
        Self::try_new(value).expect("Can't make futex!")
    }

    pub fn try_new(value: Primitive) -> Option<Self> {
        let inner = Box::new(FutexInner::new(value));

        let result = unsafe { syscall3(77, 0, (&*inner as *const FutexInner).addr()) };

        if result == 0 {
            None
        } else {
            Some(Self {
                inner,
                fd: result
            })
        }
    }
}

impl Deref for KFutex {
    type Target = FutexInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Drop for KFutex {
    fn drop(&mut self) {
        // Destroy futex.
        unsafe { syscall3(77, 1, self.fd) };
    }
}

// I should handle this descriptor-memory crap to make it const :(
pub struct Futex {
    initial_value: Primitive,
    futex: UnsafeCell<Option<KFutex>>
}

pub type SmallFutex = Futex;
pub type SmallPrimitive = Primitive;

impl Futex {
    pub const fn new(value: Primitive) -> Self {
        Self {
            initial_value: value,
            futex: UnsafeCell::new(None)
        }
    }
}

unsafe impl Sync for Futex {}
unsafe impl Send for Futex {}

impl Deref for Futex {
    type Target = FutexInner;

    fn deref(&self) -> &Self::Target {
        let ptr = self.futex.get();
        let mut ref_ = unsafe { &mut *ptr };

        if ref_.is_none() {
            *ref_ = Some(KFutex::new(self.initial_value));
        }

        &ref_.as_ref().unwrap().inner
    }
}

impl DerefMut for Futex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ptr = self.futex.get();
        let mut ref_ = unsafe { &mut *ptr };

        if ref_.is_none() {
            *ref_ = Some(KFutex::new(self.initial_value));
        }

        &mut ref_.as_mut().unwrap().inner
    }
}

/// Waits for a `futex_wake` operation to wake us.
///
/// Returns directly if the futex doesn't hold the expected value.
///
/// Returns false on timeout, and true in all other cases.
pub fn futex_wait(futex: &Futex, expected: u32, timeout: Option<Duration>) -> bool {
    let fd = unsafe { &*futex.futex.get() }.as_ref().unwrap().fd;

    let result = unsafe { syscall5(77, 2, fd, expected as _, timeout.map(|x| x.as_millis() as usize / 10).unwrap_or(0)) };

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
    let fd = unsafe { &*futex.futex.get() }.as_ref().unwrap().fd;

    let result = unsafe { syscall4(77, 3, fd, 1) };

    result > 0
}

pub fn futex_wake_all(futex: &Futex) {
    let fd = unsafe { &*futex.futex.get() }.as_ref().unwrap().fd;

    unsafe {
        syscall4(77, 3, fd, usize::MAX);
    }
}
