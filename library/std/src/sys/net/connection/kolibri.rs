use core::fmt::Write;

use crate::fmt;
use crate::io::{self, BorrowedCursor, IoSlice, IoSliceMut};
use crate::net::{IpAddr, Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr, ToSocketAddrs};
use crate::sys::pal::{api as kos_api, net as kos_net};
use crate::sys::unsupported;
use crate::time::Duration;

pub struct TcpStream {
    stream: kos_net::SocketStream
}

impl TcpStream {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<TcpStream> {
        let addr = addr.to_socket_addrs()?.next().unwrap();

        let ip = addr.ip();
        let port = addr.port();

        let real_ip = match ip {
            IpAddr::V4(addr) => addr,
            IpAddr::V6(_) => {
                writeln!(kos_api::debugboard(), "IPv6 is not yet supported...").unwrap();
                return Err(io::ErrorKind::Unsupported.into());
            }
        };

        writeln!(kos_api::debugboard(), "Addr: {real_ip:?}; Port: {port:?}").unwrap();

        let socket = kos_net::Socket::open(
            kos_net::SocketAddrFamily::Inet4,
            kos_net::SocketType::Stream,
            kos_net::SocketProtocol::Ip(kos_net::IpProtocol::Tcp),
        ).map_err(|e| e.into_io_errorkind())?;

        let stream = socket.connect_ipv4(&real_ip.octets(), port).map_err(|e| e.into_io_errorkind())?;

        Ok(TcpStream { stream })
    }

    pub fn connect_timeout(_: &SocketAddr, _: Duration) -> io::Result<TcpStream> {
        writeln!(kos_api::debugboard(), "net::connect_timeout unimplemented").unwrap();

        unsupported()
    }

    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        writeln!(kos_api::debugboard(), "net::set_read_timeout unimplemented").unwrap();
        
        unsupported()
    }

    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        writeln!(kos_api::debugboard(), "net::set_write_timeout unimplemented").unwrap();

        unsupported()
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        writeln!(kos_api::debugboard(), "net::read_timeout unimplemented").unwrap();

        unsupported()
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        writeln!(kos_api::debugboard(), "net::write_timeout unimplemented").unwrap();

        unsupported()
    }

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        writeln!(kos_api::debugboard(), "net::peek unimplemented").unwrap();

        unsupported()
    }

    pub fn read(&self, buffer: &mut [u8]) -> io::Result<usize> {
        writeln!(kos_api::debugboard(), "net::read not tested!").unwrap();

        self.stream.recv(buffer).map_err(|e| e.into_io_errorkind().into())
    }

    pub fn read_buf(&self, _buf: BorrowedCursor<'_, u8>) -> io::Result<()> {
        writeln!(kos_api::debugboard(), "net::read_buf unimplemented").unwrap();

        unsupported()
    }

    pub fn read_vectored(&self, _: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        writeln!(kos_api::debugboard(), "net::read_vectored unimplemented").unwrap();

        unsupported()
    }

    pub fn is_read_vectored(&self) -> bool {
        false
    }

    pub fn write(&self, data: &[u8]) -> io::Result<usize> {
        self.stream.send(data).map_err(|e| e.into_io_errorkind().into())
    }

    pub fn write_vectored(&self, _: &[IoSlice<'_>]) -> io::Result<usize> {
        writeln!(kos_api::debugboard(), "net::write_vectored unimplemented").unwrap();

        unsupported()
    }

    pub fn is_write_vectored(&self) -> bool {
        writeln!(kos_api::debugboard(), "net::is_write_vectored unimplemented").unwrap();

        false
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        writeln!(kos_api::debugboard(), "net::peer_addr unimplemented").unwrap();

        unsupported()
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        writeln!(kos_api::debugboard(), "net::socket_addr unimplemented").unwrap();

        unsupported()
    }

    pub fn shutdown(&self, _: Shutdown) -> io::Result<()> {
        writeln!(kos_api::debugboard(), "net::shutdown unimplemented").unwrap();

        unsupported()
    }

    pub fn duplicate(&self) -> io::Result<TcpStream> {
        writeln!(kos_api::debugboard(), "net::duplicate unimplemented").unwrap();

        unsupported()
    }

    pub fn set_linger(&self, _: Option<Duration>) -> io::Result<()> {
        writeln!(kos_api::debugboard(), "net::set_linger unimplemented").unwrap();

        unsupported()
    }

    pub fn linger(&self) -> io::Result<Option<Duration>> {
        writeln!(kos_api::debugboard(), "net::linger unimplemented").unwrap();
        unsupported()
    }

    pub fn set_keepalive(&self, _: bool) -> io::Result<()> {
        writeln!(kos_api::debugboard(), "net::set_keepalive unimplemented").unwrap();
        unsupported()
    }

    pub fn keepalive(&self) -> io::Result<bool> {
        writeln!(kos_api::debugboard(), "net::keepalive unimplemented").unwrap();
        unsupported()
    }

    pub fn set_nodelay(&self, _: bool) -> io::Result<()> {
        writeln!(kos_api::debugboard(), "net::set_nodelay unimplemented").unwrap();
        unsupported()
    }

    pub fn nodelay(&self) -> io::Result<bool> {
        writeln!(kos_api::debugboard(), "net::nodelay unimplemented").unwrap();
        unsupported()
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        writeln!(kos_api::debugboard(), "net::set_ttl unimplemented").unwrap();
        unsupported()
    }

    pub fn ttl(&self) -> io::Result<u32> {
        writeln!(kos_api::debugboard(), "net::ttl unimplemented").unwrap();
        unsupported()
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        writeln!(kos_api::debugboard(), "net::take_error unimplemented").unwrap();
        unsupported()
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        writeln!(kos_api::debugboard(), "net::set_nonblocking unimplemented").unwrap();
        unsupported()
    }
}

impl fmt::Debug for TcpStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TcpStream")
    }
}

pub struct TcpListener(!);

impl TcpListener {
    pub fn bind<A: ToSocketAddrs>(_: A) -> io::Result<TcpListener> {
        unsupported()
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        self.0
    }

    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        self.0
    }

    pub fn duplicate(&self) -> io::Result<TcpListener> {
        self.0
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        self.0
    }

    pub fn ttl(&self) -> io::Result<u32> {
        self.0
    }

    pub fn set_only_v6(&self, _: bool) -> io::Result<()> {
        self.0
    }

    pub fn only_v6(&self) -> io::Result<bool> {
        self.0
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.0
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        self.0
    }
}

impl fmt::Debug for TcpListener {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}

pub struct UdpSocket(!);

impl UdpSocket {
    pub fn bind<A: ToSocketAddrs>(_: A) -> io::Result<UdpSocket> {
        unsupported()
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.0
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        self.0
    }

    pub fn recv_from(&self, _: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.0
    }

    pub fn peek_from(&self, _: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.0
    }

    pub fn send_to(&self, _: &[u8], _: &SocketAddr) -> io::Result<usize> {
        self.0
    }

    pub fn duplicate(&self) -> io::Result<UdpSocket> {
        self.0
    }

    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        self.0
    }

    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        self.0
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        self.0
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        self.0
    }

    pub fn set_broadcast(&self, _: bool) -> io::Result<()> {
        self.0
    }

    pub fn broadcast(&self) -> io::Result<bool> {
        self.0
    }

    pub fn set_multicast_loop_v4(&self, _: bool) -> io::Result<()> {
        self.0
    }

    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        self.0
    }

    pub fn set_multicast_ttl_v4(&self, _: u32) -> io::Result<()> {
        self.0
    }

    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        self.0
    }

    pub fn set_multicast_loop_v6(&self, _: bool) -> io::Result<()> {
        self.0
    }

    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        self.0
    }

    pub fn join_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        self.0
    }

    pub fn join_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        self.0
    }

    pub fn leave_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        self.0
    }

    pub fn leave_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        self.0
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        self.0
    }

    pub fn ttl(&self) -> io::Result<u32> {
        self.0
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.0
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        self.0
    }

    pub fn recv(&self, _: &mut [u8]) -> io::Result<usize> {
        self.0
    }

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        self.0
    }

    pub fn send(&self, _: &[u8]) -> io::Result<usize> {
        self.0
    }

    pub fn connect<A: ToSocketAddrs>(&self, _: A) -> io::Result<()> {
        self.0
    }
}

impl fmt::Debug for UdpSocket {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}

pub struct LookupHost(!);

impl Iterator for LookupHost {
    type Item = SocketAddr;
    fn next(&mut self) -> Option<SocketAddr> {
        self.0
    }
}

pub fn lookup_host(_host: &str, _port: u16) -> io::Result<LookupHost> {
    unsupported()
}
