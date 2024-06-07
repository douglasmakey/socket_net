use nix::sys::socket::{accept, bind, listen, recv, send, socket, Backlog, MsgFlags, SockaddrIn};
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

    // Accept incoming connections
    let conn_fd = accept(socket_fd.as_raw_fd()).expect("Failed to accept connection");

    // echo back the received data
    let mut buf = [0u8; 1024];
    let bytes_read =
        recv(conn_fd, &mut buf, MsgFlags::empty()).expect("Failed to read from connection");
    let received_data =
        std::str::from_utf8(&buf[..bytes_read]).expect("Failed to convert received data to string");

    println!("received {} bytes", bytes_read);
    println!("bytes: {:?}", &buf[..bytes_read]);
    println!(
        "hex repr: {:?}",
        received_data
            .as_bytes()
            .iter()
            .map(|b| format!("0x{:02x}", b))
            .collect::<Vec<String>>()
    );
    println!("str repr: {:?}", received_data);

    // Echo back the received data
    let bytes_written = send(conn_fd, &buf[..bytes_read], MsgFlags::empty())
        .expect("Failed to write to connection");
    println!("Sent {} bytes back to client", bytes_written);
}
