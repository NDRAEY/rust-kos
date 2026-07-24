#[inline]
pub unsafe fn syscall1(nr: usize) -> usize {
    let mut result = 0;

    unsafe { crate::arch::asm!("int 0x40", in("eax") nr, lateout("eax") result) };

    result
}

#[inline]
pub unsafe fn syscall2(nr: usize, p1: usize) -> usize {
    let mut result = 0;

    unsafe { crate::arch::asm!("int 0x40", in("eax") nr, in("ebx") p1, lateout("eax") result) };

    result
}

#[inline]
pub unsafe fn syscall3(nr: usize, p1: usize, p2: usize) -> usize {
    let mut result = 0;

    unsafe {
        crate::arch::asm!("int 0x40", in("eax") nr, in("ebx") p1, in("ecx") p2, lateout("eax") result)
    };

    result
}

pub unsafe fn syscall4(nr: usize, p1: usize, p2: usize, p3: usize) -> usize {
    let mut result = 0;

    unsafe {
        crate::arch::asm!("int 0x40", in("eax") nr, in("ebx") p1, in("ecx") p2, in("edx") p3, lateout("eax") result)
    };

    result
}

/*
pub unsafe fn syscall5(nr: usize, p1: usize, p2: usize, p3: usize, p4: usize) -> usize {
    let mut result = 0;

    unsafe {
        crate::arch::asm!("int 0x40", in("eax") nr, in("ebx") p1, in("ecx") p2, in("edx") p3, in("esi") p4, lateout("eax") result)
    };

    result
}

pub unsafe fn syscall6(nr: usize, p1: usize, p2: usize, p3: usize, p4: usize, p5: usize) -> usize {
    let mut result = 0;

    unsafe {
        crate::arch::asm!("int 0x40", in("eax") nr, in("ebx") p1, in("ecx") p2, in("edx") p3, in("esi") p4, in("edi") p5, lateout("eax") result)
    };

    result
}
*/
