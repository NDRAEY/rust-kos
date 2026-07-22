//! Global Allocator for KolibriOS.

const PAGE_SIZE: usize = 4096;
static mut MAIN_SECTOR: usize = 0;

use crate::alloc::Layout;
use crate::ptr::null_mut;
use crate::sync::atomic::AtomicBool;
use crate::sync::atomic::Ordering;

use crate::sys::pal::api as sys;

#[allow(unused)]
#[derive(Copy, Clone)]
enum Sign {
    Dead = 0,
    Active = 1,
    Free = 2,
}

#[derive(Clone)]
struct SectorHeader {
    pub size: usize,
    pub size_left: usize,
}

#[derive(Clone)]
struct BlockHeader {
    pub sign: Sign,
    pub size: usize,
}

static HEAP_INIT: AtomicBool = AtomicBool::new(false);

fn init() {
    if !HEAP_INIT.swap(true, Ordering::Relaxed) {
        unsafe {
            sys::init_heap();
            
            MAIN_SECTOR = sys::alloc(PAGE_SIZE).addr();

            core::ptr::write_bytes(core::ptr::with_exposed_provenance_mut::<u8>(MAIN_SECTOR), 0, PAGE_SIZE); // make sector table start clean
        }
    }
}

fn malloc(size: usize) -> *mut u8 {
    unsafe {
        for i in 0..PAGE_SIZE / 4 {
            let section: *mut u32 = crate::ptr::with_exposed_provenance_mut(MAIN_SECTOR + i * 4);

            let addr: *const u8 =
                crate::ptr::with_exposed_provenance(*(section) as _);

            if !addr.is_null() {
                let sec = addr;
                let hdr = &mut *(addr as *mut SectorHeader);
                let sec_start_blocks = (sec.addr()) + size_of::<SectorHeader>();
                
                if hdr.size_left >= size {
                    let mut j = sec_start_blocks;
                    let mut first_found_block_addr = 0;

                    while j <= sec_start_blocks + hdr.size {
                        let block = &mut *(crate::ptr::with_exposed_provenance_mut::<BlockHeader>(j));
                        match block.sign {
                            Sign::Active => {
                                // If block is occupated - pass
                                first_found_block_addr = 0;
                                j += size_of::<BlockHeader>() + block.size;
                            }

                            Sign::Free => {
                                if first_found_block_addr == 0 {
                                    first_found_block_addr = j;
                                }

                                let sum_size = j - first_found_block_addr + block.size;
                                if sum_size < size {
                                    // if not enough size - pass and find next block
                                    j += (size_of::<BlockHeader>()) + block.size;
                                } else if size - sum_size < size_of::<BlockHeader>() {
                                    // Create 2 blocks
                                    let main_block =
                                        &mut *(crate::ptr::with_exposed_provenance_mut::<BlockHeader>(first_found_block_addr));
                                    main_block.sign = Sign::Active;
                                    main_block.size = size;

                                    let secondary_block =
                                        crate::ptr::with_exposed_provenance_mut::<BlockHeader>(first_found_block_addr + size_of::<BlockHeader>() + size);

                                    (*secondary_block).sign = Sign::Free;
                                    (*secondary_block).size =
                                        sum_size - size - size_of::<BlockHeader>();

                                    hdr.size_left -= size + size_of::<BlockHeader>();

                                    return crate::ptr::with_exposed_provenance_mut::<u8>(first_found_block_addr)
                                        .add(size_of::<BlockHeader>());
                                } else {
                                    // Create 1 block
                                    let main_block =
                                        &mut *(crate::ptr::with_exposed_provenance_mut::<BlockHeader>(first_found_block_addr));
                                    main_block.sign = Sign::Active;
                                    main_block.size = sum_size - size_of::<BlockHeader>();

                                    hdr.size_left -= main_block.size + size_of::<BlockHeader>();

                                    return (crate::ptr::with_exposed_provenance_mut::<u8>(first_found_block_addr))
                                        .add(size_of::<BlockHeader>());
                                }
                            }

                            Sign::Dead => {
                                // We found \0 - dead zone. There are no further blocks
                                if j + size + size_of::<BlockHeader>()
                                    <= sec_start_blocks + hdr.size
                                {
                                    // There is enough space for creating new block
                                    block.sign = Sign::Active;
                                    block.size = size;
                                    hdr.size_left -= size + size_of::<BlockHeader>();
                                    return crate::ptr::with_exposed_provenance_mut(
                                        (j + size_of::<BlockHeader>()) as _,
                                    );
                                } else {
                                    // There is not enough space, go to next sector
                                    break;
                                }
                            }
                        }
                    }
                }
            } else {
                // round requested size up to whole pages
                let sec_size = (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
                let new_sec = sys::alloc(sec_size);
                let sec_hdr = new_sec as *mut SectorHeader;

                *(section) = new_sec.addr() as _; // remember this sector in the table
                
                *sec_hdr = SectorHeader {
                    size: sec_size,
                    size_left: sec_size - size_of::<SectorHeader>(),
                };
                
                let new_block = new_sec.add(size_of::<SectorHeader>()) as *mut BlockHeader;
                
                (*new_block).sign = Sign::Active;
                (*new_block).size = size;
                (*sec_hdr).size_left -= size + size_of::<BlockHeader>();
                
                return new_block.add(1) as *mut u8;
            }
        }
    }

    panic!("Malloc error: end of the loop")
}

fn free(block: *const u8) {
    unsafe {
        let block_hdr = &mut *(block.sub(size_of::<BlockHeader>()) as *mut BlockHeader);

        for i in 0..PAGE_SIZE / 4 {
            let section: *mut u32 = crate::ptr::with_exposed_provenance_mut((MAIN_SECTOR + i * 4) as _);

            let addr: *const u32 =
                crate::ptr::with_exposed_provenance(*(section) as _);

            if addr.is_null() {
                continue;
            }
            let hdr = &mut *(addr as *mut SectorHeader);

            if addr.addr() < block.addr() && (block.addr()) < (addr.addr()) + hdr.size {
                hdr.size_left += block_hdr.size + size_of::<BlockHeader>();
                if hdr.size_left == hdr.size - size_of::<SectorHeader>() {
                    sys::free(addr.cast()); // whole sector is free, hand it back to OS

                    *section = 0; // and drop from the table
                } else {
                    block_hdr.sign = Sign::Free;
                }
                break;
            }
        }
    }
}

use super::realloc_fallback;

pub unsafe fn alloc(layout: Layout) -> *mut u8 {
    if layout.align() != 1 {
        // throw_new!("Only byte aligned available now");
        return null_mut();
    }

    init();
    malloc(layout.size())
}

pub unsafe fn dealloc(ptr: *mut u8, _: Layout) {
    // free keeps track of layout presumably????
    free(ptr)
}

pub unsafe fn realloc(ptr: *mut u8, old_layout: Layout, new_size: usize) -> *mut u8 {
    unsafe { realloc_fallback(ptr, old_layout, new_size) }
}
