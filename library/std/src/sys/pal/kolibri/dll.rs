use crate::ffi::CStr;
use crate::ffi::c_char;
use crate::sys::pal::kolibri::syscall::*;

#[repr(C)]
pub struct DLLEntry {
    raw_name: *const c_char,
    function_addr: usize
}

impl DLLEntry {
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.raw_name) }
    }

    pub fn data<T>(&self) -> &T {
        unsafe {
            &*(&self.function_addr as *const _ as *const T)
        }
    }
}

pub struct DLLTable {
    entries: *const DLLEntry
}

impl DLLTable {
    pub fn iter(&self) -> DLLTableIterator<'_> {
        DLLTableIterator {
            dll: self,
            position: 0
        }
    }
}

pub struct DLLTableIterator<'dll> {
    dll: &'dll DLLTable,
    position: usize
}

impl Iterator for DLLTableIterator<'_> {
    type Item = DLLEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let this = unsafe { self.dll.entries.add(self.position).read_unaligned() };

        if this.raw_name.is_null() {
            return None;
        }

        self.position += 1;

        Some(this)
    }
}

pub fn load_dll(path: &crate::ffi::CStr) -> Option<DLLTable> {
    let value = unsafe { syscall3(68, 19, path.as_ptr().addr() as _) };

    if value == 0 {
        return None;
    }

    Some(DLLTable {
        entries: crate::ptr::with_exposed_provenance::<DLLEntry>(value)
    })
}