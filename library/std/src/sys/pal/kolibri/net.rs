use super::syscall::*;
use crate::ffi::c_char;

pub type SocketDescriptor = usize;

// https://git.kolibrios.org/KolibriOS/kolibrios/src/commit/0ca124d6cb81f5a4d609935337000a031ac22287/kernel/trunk/network/socket.inc#L330
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SocketAddrFamily {
    Unspecified = 0,
    Local = 1,
    Inet4 = 2,
    Inet6 = 3,
}

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SocketType {
    Stream = 1,
    Dgram = 2,
    Raw = 3,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SocketProtocol {
    Ip(IpProtocol),
}

impl SocketProtocol {
    pub fn into_raw(self) -> usize {
        match self {
            Self::Ip(prot) => prot as usize,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum IpProtocol {
    Ip = 0,
    Icmp = 1,
    Tcp = 6,
    Udp = 17,
    Raw = 255,
}

// https://git.kolibrios.org/KolibriOS/kolibrios/src/commit/0ca124d6cb81f5a4d609935337000a031ac22287/kernel/trunk/network/socket.inc#L173#[repr(C)]
#[repr(C, packed(1))]
pub struct SocketAddr {
    pub family: SocketAddrFamily,
    pub port: u16,
    pub ip: u32,

    _reserved: [u8; 8],
}

const _: &[()] = &[
    assert!(core::mem::offset_of!(SocketAddr, family) == 0),
    assert!(core::mem::offset_of!(SocketAddr, port) == 2),
    assert!(core::mem::offset_of!(SocketAddr, ip) == 4),
    assert!(core::mem::offset_of!(SocketAddr, _reserved) == 8),
];

impl SocketAddr {
    pub fn new(family: SocketAddrFamily, ip: u32, port: u16) -> Self {
        Self {
            family, port, ip, _reserved: Default::default()
        }
    }
}

pub struct Socket {
    sd: SocketDescriptor,
    family: SocketAddrFamily,
    stype: SocketType,
    proto: SocketProtocol,
}

use core::fmt::Write;

impl Socket {
    pub fn open(
        family: SocketAddrFamily,
        stype: SocketType,
        proto: SocketProtocol,
    ) -> super::error::NetworkResult<Self> {
        let (socket_nr, error) =
            unsafe { syscall5_2(75, 0, family as _, stype as _, proto.into_raw()) };

        writeln!(super::api::debugboard(), "net: Family: {family:?}; Type: {stype:?}; Protocol: {proto:?}").unwrap();
        writeln!(super::api::debugboard(), "net: Socket number: {socket_nr:x}; Error: {error}").unwrap();

        if (socket_nr as i32) == -1 {
            Err(unsafe { core::mem::transmute(error) })
        } else {
            Ok(Socket {
                sd: socket_nr,
                family,
                stype,
                proto
            })
        }
    }

    // For use in `Drop`.
    fn socket_close(&self) -> super::error::NetworkResult<()> {
        let (is_error, error) = unsafe { syscall3_2(75, 1, self.sd) };

        if (is_error as i32) == -1 {
            Err(unsafe { core::mem::transmute(error) })
        } else {
            Ok(())
        }
    }

    pub fn bind_ipv4(&self, addr: &[u8; 4], port: u16) -> super::error::NetworkResult<()> {
        let sockaddr = SocketAddr::new(SocketAddrFamily::Inet4, u32::from_le_bytes(*addr), port.to_be());
        let sa_len = core::mem::size_of_val(&sockaddr);

        let (is_error, error) = unsafe { syscall5_2(75, 2, self.sd, (&sockaddr as *const SocketAddr).addr(), sa_len) };

        if (is_error as i32) == -1 {
            Err(unsafe { core::mem::transmute(error) })
        } else {
            Ok(())
        }
    }

    pub fn connect_ipv4(self, addr: &[u8; 4], port: u16) -> super::error::NetworkResult<SocketStream> {
        let sockaddr = SocketAddr::new(SocketAddrFamily::Inet4, u32::from_le_bytes(*addr), port.to_be());
        let sa_len = core::mem::size_of_val(&sockaddr);

        let (is_error, error) = unsafe { syscall5_2(75, 4, self.sd, (&sockaddr as *const SocketAddr).addr(), sa_len) };

        writeln!(super::api::debugboard(), "net: Connect socket {:?} to address {addr:?} port {port}", self.sd).unwrap();
        writeln!(super::api::debugboard(), "net: error indicator: {is_error}, error: {error}").unwrap();

        if (is_error as i32) == -1 {
            Err(unsafe { core::mem::transmute(error) })
        } else {
            Ok(SocketStream { socket: self })
        }
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        let result = self.socket_close();

        writeln!(super::api::debugboard(), "net: Socket {} is dropped and therefore closed (result {result:?}).", self.sd).unwrap();
    }
}

pub struct SocketStream {
    socket: Socket
}

impl SocketStream {
    pub fn send(&self, data: &[u8]) -> super::error::NetworkResult<usize> {
        let (bytes_copied, error) = unsafe { syscall6_2(75, 6, self.socket.sd, data.as_ptr().addr(), data.len(), 0) };
        
        writeln!(super::api::debugboard(), "net: Send {} bytes of data result {bytes_copied}, error: {error}", data.len()).unwrap();

        // -1 means error.
        if bytes_copied == usize::MAX {
            Err(unsafe { core::mem::transmute(error) })
        } else {
            Ok(bytes_copied)
        }
    }

    pub fn recv(&self, buffer: &mut [u8]) -> super::error::NetworkResult<usize> {
        let (bytes_copied, error) = unsafe { syscall6_2(75, 7, self.socket.sd, buffer.as_ptr().addr(), buffer.len(), 0) };
        
        writeln!(super::api::debugboard(), "net: Receive {} bytes of data result {bytes_copied}, error: {error}", buffer.len()).unwrap();

        // -1 means error.
        if bytes_copied == usize::MAX {
            Err(unsafe { core::mem::transmute(error) })
        } else {
            Ok(bytes_copied)
        }
    }
}