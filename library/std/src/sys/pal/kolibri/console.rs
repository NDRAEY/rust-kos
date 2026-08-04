use super::api::*;
use super::shm::*;

use crate::ffi::CString;

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
enum ShellCommands {
    Ok = 0,
    Exit = 1,
    Putc = 2,
    Puts = 3,
    Getc = 4,
    Gets = 5,
    Cls = 6,
    Pid = 7,
    Ping = 8
}

pub struct Console {
    shm_name: CString,
    buffer_size: usize,
    is_fully_initialized: bool,
    shm_area: *mut u8
}

unsafe impl Sync for Console {}
unsafe impl Send for Console {}

impl Console {
    pub fn new() -> Option<Self> {
        let pid = get_proc_info(None).unwrap().pid_tid;

        eprintln!("PID={pid:?}");

        let mut shm_name = CString::new(pid.to_string() + "-SHELL").unwrap();

        eprintln!("SHM name is: {shm_name:?}");

        let (shm_area, shm_size) = shm_open(&shm_name, 16 << 10, SHM_OPEN_ALWAYS | SHM_WRITE).unwrap();

        eprintln!("SHM area at: {shm_area:?} with size {shm_size:?}");

        let mut this = Self {
            shm_name,
            buffer_size: 10 << 16,
            is_fully_initialized: false,
            shm_area: shm_area.into()
        };

        this.ping()?;

        this.is_fully_initialized = true;

        Some(this)
    }

    fn ping(&mut self) -> Option<()> {
        self.write_shm(&[ShellCommands::Ping as u8]);

        yield_now();

        for _ in 0..10 {
            if self.read_shm_byte() == ShellCommands::Ok as u8 {
                return Some(());
            }
            
            sleep_this_thread(crate::time::Duration::from_millis(10).as_nanos() as _);        
        }

        None
    }

    fn write_shm(&mut self, data: &[u8]) {
        unsafe { self.shm_area.copy_from_nonoverlapping(data.as_ptr(), data.len()) };
    }

    fn write_shm_offset(&mut self, offset: isize, data: &[u8]) {
        unsafe { self.shm_area.offset(offset).copy_from_nonoverlapping(data.as_ptr(), data.len()) };
    }

    fn read_shm_byte(&self) -> u8 {
        unsafe { self.shm_area.read_volatile() }
    }

    fn read_shm(&self, buf: &mut [u8]) {
        unsafe { self.shm_area.copy_to_nonoverlapping(buf.as_mut_ptr(), buf.len()) };
    }

    fn read_shm_offset(&self, offset: isize, buf: &mut [u8]) {
        unsafe { self.shm_area.copy_to_nonoverlapping(buf.as_mut_ptr(), buf.len()) };
    }

    pub fn putc(&mut self, char: u8) {
        self.ping();

        self.write_shm(&[ShellCommands::Putc as u8, char]);

        self.wait()
    }

    pub fn write(&mut self, s: &[u8]) {
        self.ping();
        
        self.write_shm(&[ShellCommands::Puts as u8]);

        self.write_shm_offset(1, s);
        
        self.write_shm_offset((1 + s.len()) as _, &[0]);

        self.wait()
    }

    pub fn puts(&mut self, s: &str) {
        self.write(s.as_bytes())
    }

    pub fn getc(&mut self) -> u8 {
        self.ping();
        
        self.write_shm(&[ShellCommands::Getc as u8]);

        self.wait();

        let mut buf = [0u8];

        self.read_shm_offset(1, &mut buf);

        buf[0]
    }

    fn wait(&self) {
        while self.read_shm_byte() != ShellCommands::Ok as u8 {
            sleep_this_thread(crate::time::Duration::from_millis(10).as_nanos() as _);
        }
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        if self.is_fully_initialized {
            self.write_shm(&[ShellCommands::Exit as u8]);
            self.wait();
        }

        shm_close(&self.shm_name);
    }
}