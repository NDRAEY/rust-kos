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

// Used in syscalls that return multiple values.
#[inline]
pub unsafe fn syscall2_all(nr: usize, p1: usize) -> (usize, usize) {
    let mut eax = 0;
    let mut ebx = 0;

    unsafe {
        crate::arch::asm!("int 0x40", in("eax") nr, in("ebx") p1, lateout("eax") eax, lateout("ebx") ebx)
    };

    (eax, ebx)
}

#[inline]
pub unsafe fn syscall3(nr: usize, p1: usize, p2: usize) -> usize {
    let mut result = 0;

    unsafe {
        crate::arch::asm!("int 0x40", in("eax") nr, in("ebx") p1, in("ecx") p2, lateout("eax") result)
    };

    result
}

#[inline]
pub unsafe fn syscall3_2(nr: usize, p1: usize, p2: usize) -> (usize, usize) {
    let mut eax = 0;
    let mut ebx = 0;

    unsafe {
        crate::arch::asm!("int 0x40", in("eax") nr, in("ebx") p1, in("ecx") p2, lateout("eax") eax, lateout("ebx") ebx)
    };

    (eax, ebx)
}

pub unsafe fn syscall4(nr: usize, p1: usize, p2: usize, p3: usize) -> usize {
    let mut result = 0;

    unsafe {
        crate::arch::asm!("int 0x40", in("eax") nr, in("ebx") p1, in("ecx") p2, in("edx") p3, lateout("eax") result)
    };

    result
}

pub unsafe fn syscall5(nr: usize, p1: usize, p2: usize, p3: usize, p4: usize) -> usize {
    let eax: usize;
 
    unsafe {
        core::arch::asm!(
            "push esi",
            "mov esi, {p4}",
            "int 0x40",
            "pop esi",
            p4 = in(reg) p4,
            inlateout("eax") nr => eax,
            in("ecx") p2,
            in("edx") p3,
        );
    }

    eax
}

pub unsafe fn syscall5_2(nr: usize, p1: usize, p2: usize, p3: usize, p4: usize) -> (usize, usize) {
    let eax: usize;
    let ebx: usize;
    unsafe {
        core::arch::asm!(
            "push esi",
            "mov esi, {p4}",
            "int 0x40",
            "pop esi",
            p4 = in(reg) p4,
            inlateout("eax") nr => eax,
            inlateout("ebx") p1 => ebx,
            in("ecx") p2,
            in("edx") p3,
        );
    }
    (eax, ebx)
}

pub unsafe fn syscall6_2(
    nr: usize, p1: usize, p2: usize, p3: usize, p4: usize, p5: usize,
) -> (usize, usize) {
    let eax: usize;
    let ebx: usize;
    unsafe {
        core::arch::asm!(
            "push esi",
            "push edi",
            "mov esi, {p4}",
            "mov edi, {p5}",
            "int 0x40",
            "pop edi",
            "pop esi",
            p4 = in(reg) p4,
            p5 = in(reg) p5,
            inlateout("eax") nr => eax,
            inlateout("ebx") p1 => ebx,
            in("ecx") p2,
            in("edx") p3,
        );
    }
    (eax, ebx)
}