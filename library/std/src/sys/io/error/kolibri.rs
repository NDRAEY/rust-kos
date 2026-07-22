use crate::io;

pub fn errno() -> io::RawOsError {
    0
}

pub fn is_interrupted(_code: io::RawOsError) -> bool {
    false
}

pub fn decode_error_kind(_code: io::RawOsError) -> io::ErrorKind {
    io::ErrorKind::Uncategorized
}

pub fn error_string(errno: io::RawOsError) -> String {
    format!("Status: {errno}")
}
