use nix::sys::socket::{connect, send, socket, MsgFlags, SockaddrIn};
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
        SockaddrIn::from_str("127.0.0.1:8000").expect("Failed to create socket address");

    // Connect to the server
    connect(socket_fd.as_raw_fd(), &sock_addr).expect("Failed to connect to server");

    // Read the file into a buffer
    let buffer = std::fs::read("./src/data/data.txt").expect("Failed to read file");

    // send the size of the file to the server
    let size: u32 = buffer.len() as u32;
    println!("Sending file size: {}", size);
    send(
        socket_fd.as_raw_fd(),
        &size.to_ne_bytes(),
        MsgFlags::empty(),
    )
    .expect("Failed to send data to server");

    // send the file to the server
    println!("Sending file data");
    send(socket_fd.as_raw_fd(), &buffer, MsgFlags::empty()).expect("Failed to send data to server");
}
