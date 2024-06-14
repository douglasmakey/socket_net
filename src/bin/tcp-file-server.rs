use nix::sys::socket::{accept, recv, MsgFlags};
use socket_net::server::create_tcp_server_socket;
use std::os::fd::AsRawFd;

fn main() {
    let socket_fd =
        create_tcp_server_socket("127.0.0.1:8000").expect("Failed to create server socket");
    // Accept incoming connections
    let conn_fd = accept(socket_fd.as_raw_fd()).expect("Failed to accept connection");

    // Receive the size of the file
    let mut size_buf = [0; 4];
    recv(conn_fd, &mut size_buf, MsgFlags::empty()).expect("Failed to read from connection");

    let file_size = u32::from_ne_bytes(size_buf);
    println!("File size: {}", file_size);

    // Receive the file data
    let mut file_buf = vec![0; file_size as usize];
    let bytes_read =
        recv(conn_fd, &mut file_buf, MsgFlags::empty()).expect("Failed to read from connection");

    println!("File data bytes read: {:?}", bytes_read);
}
