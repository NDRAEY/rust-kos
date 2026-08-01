// use crate::ptr;

use core::sync::atomic::{AtomicU8, Ordering};

use crate::time::SystemTime;

static SEED: AtomicU8 = AtomicU8::new(42);

pub fn emit_u8() -> u8 {
    let unixtime =
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map(|x| x.as_secs()).unwrap_or(0);

    let d1 = (unixtime & 0xff_00_00_00) >> 24;
    let d2 = (unixtime & 0x00_ff_00_00) >> 16;
    let d3 = (unixtime & 0x00_00_ff_00) >> 8;
    let d4 = unixtime & 0x00_00_00_ff;

    let d1 = d1 as u8;
    let d2 = d2 as u8;
    let d3 = d3 as u8;
    let d4 = d4 as u8;

    let mask = d1 ^ d2 ^ d3 ^ d4;

    SEED.fetch_add(mask, Ordering::Relaxed);

    SEED.load(Ordering::Relaxed)
}

pub fn fill_bytes(bytes: &mut [u8]) {
    for byte in bytes {
        *byte = emit_u8();
    }
}

// pub fn hashmap_random_keys() -> (u64, u64) {
//     // Use allocation addresses for a bit of randomness. This isn't
//     // particularly secure, but there isn't really an alternative.
//     let stack = 0u8;
//     let heap = Box::new(0u8);
//     let k1 = ptr::from_ref(&stack).addr() as u64;
//     let k2 = ptr::from_ref(&*heap).addr() as u64;
//     (k1, k2)
// }
