use super::syscall::*;

#[inline]
pub fn exit(_code: u32) -> ! {
    unsafe { syscall1(0xffff_ffff) };

    unreachable!()
}

#[inline]
pub fn init_heap() {
    unsafe { syscall2(68, 11) };
}

#[inline]
pub fn alloc(size: usize) -> *mut u8 {
    unsafe { crate::ptr::with_exposed_provenance_mut(syscall3(68, 12, size)) }
}

#[inline]
pub fn realloc(ptr: *mut u8, size: usize) -> *mut u8 {
    unsafe { crate::ptr::with_exposed_provenance_mut(syscall4(68, 20, ptr.addr() as _, size)) }
}

#[inline]
pub fn free(ptr: *mut u8) {
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

pub struct DebugBoard;

impl crate::fmt::Write for DebugBoard {
    fn write_str(&mut self, s: &str) -> crate::fmt::Result {
        for i in s.bytes() {
            debugboard_write(i);
        }

        Ok(())
    }
}

pub fn debugboard() -> DebugBoard {
    DebugBoard
}

fn bcd2dec(bcd: u8) -> u8 {
    bcd - 6 * (bcd >> 4)
}

pub fn time() -> super::time::Time {
    let value = unsafe { syscall1(3) };

    let bcd_h = value & 0xff;
    let bcd_m = (value >> 8) & 0xff;
    let bcd_s = (value >> 16) & 0xff;

    super::time::Time {
        hour: bcd2dec(bcd_h as u8),
        minute: bcd2dec(bcd_m as u8),
        second: bcd2dec(bcd_s as u8),
    }
}

pub fn date() -> super::time::Date {
    let value = unsafe { syscall1(29) };

    let bcd_y = value & 0xff;
    let bcd_m = (value >> 8) & 0xff;
    let bcd_d = (value >> 16) & 0xff;

    super::time::Date {
        year: bcd2dec(bcd_y as u8) as u16 + 2000,
        month: bcd2dec(bcd_m as u8),
        day: bcd2dec(bcd_d as u8),
    }
}

pub fn yield_now() {
    unsafe { syscall2(68, 1) };
}

pub fn sleep_this_thread(nanos: u128) {
    let millis = nanos / 1_000_000;

    // https://wiki.kolibrios.org/wiki/SysFn05/ru
    // ebx = time in hundredths of a second.
    //
    // It means that sleep resolution is 10 ms.
    // TODO: Wait when KOS devs fix that, making sleep resolution at least 1 ms (would be perfectly if 1 ns).
    unsafe { syscall2(5, (millis / 10) as _) };
}

pub fn spawn_thread(entry_point: *const (), stack_top: *mut usize) -> usize {
    unsafe { syscall4(51, 1, entry_point.addr(), stack_top.addr()) }
}

#[repr(C, packed(1))]
pub struct ProcessInfo {
    pub ticks: usize,
    pub window_position: u16,
    pub slot_nr: u16,
    pub rsv0: u16,
    pub process_name: [core::ffi::c_char; 11],
    pub rsv1: u8,
    pub proc_address: u32,
    pub memory_used: u32,
    pub pid_tid: u32,
    pub window_x: u32,
    pub window_y: u32,
    pub window_width: u32,
    pub window_height: u32,
    pub slot_state: u16,
    pub rsv2: u16,
    pub client_area_x: u32,
    pub client_area_y: u32,
    pub client_area_width: u32,
    pub client_area_height: u32,
    pub window_state: u8,
    pub event_mask: u32,
    pub keyboard_mode: u8,
}

pub fn get_proc_info(slot_nr: Option<usize>) -> Option<ProcessInfo> {
    // let mut proc_info: ProcessInfo = unsafe { core::mem::zeroed() };
    let mut proc_info = [0_u32; 256];

    let result = unsafe {
        syscall3(
            9,
            proc_info.as_mut_ptr().addr(),
            slot_nr.map(|x| x as isize).unwrap_or(-1) as _,
        )
    };

    if result == usize::MAX {
        return None;
    }

    Some(unsafe { proc_info.as_ptr().cast::<ProcessInfo>().read() })
}
