use nix::sys::socket::{
    bind, connect, listen, socket, AddressFamily, Backlog, SockFlag, SockProtocol, SockType,
    SockaddrIn,
};
use std::{os::fd::AsRawFd, str::FromStr};

fn main() {
    println!("Testing backlog...");
    for backlog_size in 0..5 {
        let server_socket = socket(
            AddressFamily::Inet,
            SockType::Stream,
            SockFlag::empty(),
            SockProtocol::Tcp,
        )
        .expect("Failed to create server socket");

        let server_address =
            SockaddrIn::from_str("127.0.0.1:8080").expect("Failed to create server address");
        bind(server_socket.as_raw_fd(), &server_address).expect("Failed to bind server socket");

        listen(&server_socket, Backlog::new(backlog_size).unwrap())
            .expect("Server socket listen fails");
        let mut successful_connections = vec![];
        loop {
            let client_socket = socket(
                AddressFamily::Inet,
                SockType::Stream,
                SockFlag::empty(),
                SockProtocol::Tcp,
            )
            .expect("Client socket creation fails");

            match connect(client_socket.as_raw_fd(), &server_address) {
                Ok(_) => successful_connections.push(client_socket),
                Err(_) => {
                    break;
                }
            }
        }

        println!(
            "Backlog {} successful connections: {}",
            backlog_size,
            successful_connections.len()
        );
    }
}
