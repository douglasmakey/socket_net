use nix::sys::socket::{connect, recv, send, socket, MsgFlags, SockaddrIn};
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

    // Connect to the server
    connect(socket_fd.as_raw_fd(), &sock_addr).expect("Failed to connect to server");

    println!("Connected to server at address: {}", sock_addr);

    // Send data to the server
    let data = "Hello from KungfuDev";
    send(socket_fd.as_raw_fd(), data.as_bytes(), MsgFlags::empty())
        .expect("Failed to send data to server");

    println!("Sent data to server: {}", data);

    // Receive data from the server
    let mut buf = [0u8; 1024];
    let bytes_read = recv(socket_fd.as_raw_fd(), &mut buf, MsgFlags::empty())
        .expect("Failed to read from connection");

    let received_data =
        std::str::from_utf8(&buf[..bytes_read]).expect("Failed to convert received data to string");

    println!("Received from server: {}", received_data);
}
