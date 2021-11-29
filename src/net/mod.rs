//! Network-related operations.
//!
//! On Windows, one must call [`wsa_startup`] in the process before calling any
//! of these APIs. [`wsa_cleanup`] may be used in the process if these APIs are
//! no longer needed.
//!
//! [`wsa_startup`]: https://docs.rs/rustix/latest/x86_64-pc-windows-msvc/rustix/net/fn.wsa_startup.html
//! [`wsa_cleanup`]: https://docs.rs/rustix/latest/x86_64-pc-windows-msvc/rustix/net/fn.wsa_cleanup.html

use crate::imp;

mod send_recv;
mod socket;
mod socket_addr_any;
#[cfg(not(any(windows, target_os = "wasi")))]
mod socketpair;
#[cfg(windows)]
mod wsa;

pub mod sockopt;

#[cfg(not(windows))]
pub use send_recv::sendto_unix;
pub use send_recv::{recv, recvfrom, send, sendto_v4, sendto_v6, RecvFlags, SendFlags};
pub use socket::{
    accept, accept_with, acceptfrom, acceptfrom_with, bind, bind_any, bind_v4, bind_v6, connect,
    connect_any, connect_v4, connect_v6, getpeername, getsockname, listen, shutdown, socket,
    socket_with, AcceptFlags, AddressFamily, Protocol, Shutdown, SocketFlags, SocketType,
};
#[cfg(not(windows))]
pub use socket::{bind_unix, connect_unix};
pub use socket_addr_any::SocketAddrAny;
#[cfg(not(any(windows, target_os = "wasi")))]
pub use socketpair::socketpair;
#[cfg(windows)]
pub use wsa::{wsa_cleanup, wsa_startup};

pub use imp::net::SocketAddrStorage;
#[cfg(not(windows))]
pub use imp::net::SocketAddrUnix;

// Declare the `Ip` and `Socket` address types.
#[cfg(all(ip, not(feature = "std")))]
pub use core::net::Ipv6MulticastScope;
#[cfg(not(feature = "std"))]
pub use core::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
#[cfg(feature = "std")]
pub use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};
