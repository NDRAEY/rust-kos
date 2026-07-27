#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    // 0 is OK.
    
    // 2 = Function is not supported by filesystem.
    Unsupported = 2,
    // 3 = Unknown filesystem
    UnknownFilesystem = 3,
    // 5 = File not found
    NotFound = 5,
    // 6 = File EOF
    FileEof = 6,
    // 7 = Pointer is outside of process memory
    InvalidMemory = 7,
    // 8 = Disk is full
    StorageFull = 8,
    // 9 = Filesystem error
    Filesystem = 9,
    // 10 = Permission denied
    PermissionDenied = 10,
    // 10 = I/O error
    Io = 11,
    // 12 = Filesystem out of RAM
    FilesystemOutOfRam = 12,

    // 30 = Out of memory
    OutOfMemory = 30,
    // 31 = File is not an executable
    FileNotAnExec = 31,
    // 32 = Too many processes
    TooManyProcesses = 32,
}

impl Error {
    pub fn into_io_errorkind(self) -> crate::io::ErrorKind {
        crate::sys::io::decode_error_kind(self as _)
    }
}

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NetworkError {
    NoBufs = 1,
    InProgress = 2,
    OptionNotSupported = 4,
    WouldBlock = 6,
    NotConnected = 9,
    Already = 10,
    InvalidArgument = 11,
    MessageSize = 12,
    NoMemory = 18,
    AddressInUse = 20,
    AddressNotAvailable = 21,
    ConnectionReset = 52,
    ConnectionAborted = 53,
    IsConnection = 56,
    TimedOut = 60,
    ConnectionRefused = 61
}

impl NetworkError {
    pub fn into_io_errorkind(self) -> crate::io::ErrorKind {
        crate::sys::io::decode_error_kind(128 + (self as i32))
    }
}

pub type SystemResult<T> = core::result::Result<T, Error>;
pub type NetworkResult<T> = core::result::Result<T, NetworkError>;