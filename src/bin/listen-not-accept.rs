use nix::sys::socket::{bind, listen, socket, Backlog, SockaddrIn};
use std::os::unix::io::AsRawFd;
use std::str::FromStr;

fn main() {
    let socket_fd = socket(
        nix::sys::socket::AddressFamily::Inet, // Socket family
        nix::sys::socket::SockType::Stream,    // Socket type
        nix::sys::socket::SockFlag::empty(),
        None,
    )
    .expect("Failed to create socket");

    println!("Socket file descriptor: {}", socket_fd.as_raw_fd());

    // Create a socket address
    let sock_addr =
        SockaddrIn::from_str("127.0.0.1:6797").expect("Failed to create socket address");

    // Bind the socket to the address
    bind(socket_fd.as_raw_fd(), &sock_addr).expect("Failed to bind socket");

    println!("Socket bound to address: {}", sock_addr);

    // Listen for incoming connections
    let backlog = Backlog::new(1).expect("Failed to create backlog");
    listen(&socket_fd, backlog).expect("Failed to listen for connections");

    loop {}
}
