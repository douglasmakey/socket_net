use nix::sys::socket::{bind, listen, socket, Backlog, SockaddrIn};
use std::{
    os::fd::{AsRawFd, OwnedFd},
    str::FromStr,
};

pub fn create_tcp_server_socket(addr: &str) -> Result<OwnedFd, nix::Error> {
    let socket_fd = socket(
        nix::sys::socket::AddressFamily::Inet, // Socket family
        nix::sys::socket::SockType::Stream,    // Socket type
        nix::sys::socket::SockFlag::empty(),
        None,
    )?;

    println!("Socket file descriptor: {}", socket_fd.as_raw_fd());

    // Create a socket address
    let sock_addr = SockaddrIn::from_str(addr).expect("Failed to create socket address");

    // Bind the socket to the address
    bind(socket_fd.as_raw_fd(), &sock_addr)?;

    println!("Socket bound to address: {}", sock_addr);

    // Listen for incoming connections
    let backlog = Backlog::new(1).expect("Failed to create backlog");
    listen(&socket_fd, backlog)?;
    Ok(socket_fd)
}
