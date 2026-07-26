use crate::io;

pub fn errno() -> io::RawOsError {
    0
}

pub fn is_interrupted(_code: io::RawOsError) -> bool {
    false
}

// TODO: Move those error codes into list of named i32 constants for convinience.
pub fn decode_error_kind(code: io::RawOsError) -> io::ErrorKind {
    match code {
        // 0 is OK.

        // 2 = Function is not supported by filesystem.
        2 => io::ErrorKind::Unsupported,
        // 3 = Unknown filesystem 
        3 => io::ErrorKind::Other,
        // 5 = File not found
        5 => io::ErrorKind::NotFound,
        // 6 = File EOF
        6 => io::ErrorKind::UnexpectedEof,
        // 7 = Pointer is outside of process memory
        7 => io::ErrorKind::InvalidData,
        // 8 = Disk is full
        8 => io::ErrorKind::StorageFull,
        // 9 = Filesystem error
        9 => io::ErrorKind::Other,
        // 10 = Permission denied
        10 => io::ErrorKind::PermissionDenied,
        // 10 = I/O error
        11 => io::ErrorKind::InputOutputError,
        // 12 = Filesystem out of RAM
        12 => io::ErrorKind::OutOfMemory,

        // 30 = Out of memory
        30 => io::ErrorKind::OutOfMemory,
        // 31 = File is not an executable
        31 => io::ErrorKind::InvalidData,
        // 32 = Too many processes
        32 => io::ErrorKind::Other,

        _ => todo!("Unexpected error code: {code:?}")
    }
}

pub fn error_string(errno: io::RawOsError) -> String {
    format!("Status: {errno}")
}
