use crate::io::{self, BorrowedCursor, IoSlice, IoSliceMut};

pub struct Stdin;
pub struct Stdout;
pub struct Stderr;

use crate::sync::atomic::AtomicBool;

use crate::ffi::c_char;
use crate::sys::pal::dll;

struct Console {
    con_init: unsafe extern "stdcall" fn(u32, u32, u32, u32, *const c_char),
    con_write_string: unsafe extern "stdcall" fn(*const c_char, u32),
    con_exit: unsafe extern "stdcall" fn(bool),

    shown: AtomicBool
}

impl Console {
    pub fn init_from_lib() -> Option<Self> {
        let lib = dll::load_dll(c"/sys/lib/console.obj")?;

        let con_init = lib.iter().find(|en| en.name() == c"con_init")?;
        let con_write_string = lib.iter().find(|en| en.name() == c"con_write_string")?;
        let con_exit = lib.iter().find(|en| en.name() == c"con_exit")?;

        Some(Console {
            con_init: *con_init.data(),
            con_write_string: *con_write_string.data(),
            con_exit: *con_exit.data(),

            shown: AtomicBool::new(false)
        })
    }

    fn ensure_initialized(&self) {
        const DEFAULT: u32 = 0xffff_ffff;

        if !self.shown.load(crate::sync::atomic::Ordering::Acquire) {
            unsafe { (self.con_init)(DEFAULT, DEFAULT, DEFAULT, DEFAULT, c"Rust Console".as_ptr()) };

            self.shown.store(true, crate::sync::atomic::Ordering::Release);
        }
    }

    pub fn write(&self, data: &[u8]) {
        self.ensure_initialized();

        unsafe { (self.con_write_string)(data.as_ptr().cast(), data.len() as _) };
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        if self.shown.load(crate::sync::atomic::Ordering::Acquire) {
            unsafe { (self.con_exit)(false) };
        }
    }
}

static GLOBAL_CONSOLE: crate::sync::OnceLock<Console> = crate::sync::OnceLock::new();

impl Stdin {
    pub const fn new() -> Stdin {
        Stdin
    }
}

impl io::Read for Stdin {
    #[inline]
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Ok(0)
    }

    #[inline]
    fn read_buf(&mut self, _cursor: BorrowedCursor<'_, u8>) -> io::Result<()> {
        Ok(())
    }

    #[inline]
    fn read_vectored(&mut self, _bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        Ok(0)
    }

    #[inline]
    fn is_read_vectored(&self) -> bool {
        // Do not force `Chain<Empty, T>` or `Chain<T, Empty>` to use vectored
        // reads, unless the other reader is vectored.
        false
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        if !buf.is_empty() { Err(io::Error::READ_EXACT_EOF) } else { Ok(()) }
    }

    #[inline]
    fn read_buf_exact(&mut self, cursor: BorrowedCursor<'_, u8>) -> io::Result<()> {
        if cursor.capacity() != 0 { Err(io::Error::READ_EXACT_EOF) } else { Ok(()) }
    }

    #[inline]
    fn read_to_end(&mut self, _buf: &mut Vec<u8>) -> io::Result<usize> {
        Ok(0)
    }

    #[inline]
    fn read_to_string(&mut self, _buf: &mut String) -> io::Result<usize> {
        Ok(0)
    }
}

impl Stdout {
    pub const fn new() -> Stdout {
        Stdout
    }
}

impl io::Write for Stdout {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        GLOBAL_CONSOLE.get_or_init(|| Console::init_from_lib().expect("Console initialization failed!"));
        
        GLOBAL_CONSOLE.get().unwrap().write(buf);

        Ok(buf.len())
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        let total_len = bufs.iter().map(|b| b.len()).sum();
        Ok(total_len)
    }

    #[inline]
    fn is_write_vectored(&self) -> bool {
        true
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        GLOBAL_CONSOLE.get_or_init(|| Console::init_from_lib().expect("Console initialization failed!"));

        GLOBAL_CONSOLE.get().unwrap().write(buf);

        Ok(())
    }

    #[inline]
    fn write_all_vectored(&mut self, _bufs: &mut [IoSlice<'_>]) -> io::Result<()> {
        Ok(())
    }

    // Keep the default write_fmt so the `fmt::Arguments` are still evaluated.

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Stderr {
    pub const fn new() -> Stderr {
        Stderr
    }
}

impl io::Write for Stderr {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        crate::sys::pal::api::debugboard_write_bulk(buf);

        Ok(buf.len())
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        let total_len = bufs.iter().map(|b| b.len()).sum();
        Ok(total_len)
    }

    #[inline]
    fn is_write_vectored(&self) -> bool {
        true
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        crate::sys::pal::api::debugboard_write_bulk(buf);

        Ok(())
    }

    #[inline]
    fn write_all_vectored(&mut self, _bufs: &mut [IoSlice<'_>]) -> io::Result<()> {
        Ok(())
    }

    // Keep the default write_fmt so the `fmt::Arguments` are still evaluated.

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub const STDIN_BUF_SIZE: usize = 0;

pub fn is_ebadf(_err: &io::Error) -> bool {
    false
}

pub fn panic_output() -> Option<impl io::Write> {
    Some(Stderr::new())
}
