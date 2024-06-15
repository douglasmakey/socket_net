use nix::{libc::sockaddr_in, sys::socket::SockaddrIn};
use std::str::FromStr;

fn main() {
    // Create a socket address
    let sock_addr =
        SockaddrIn::from_str("127.0.0.1:6797").expect("Failed to create socket address");

    let sockaddr: sockaddr_in = sock_addr.into();
    println!("sockaddr: {:?}", sockaddr);
    println!("s_addr Default: {}", sockaddr.sin_addr.s_addr);
    // big endian
    println!("s_addr be: {:?}", sockaddr.sin_addr.s_addr.to_be());
    // little endian
    println!("s_addr le: {:?}", sockaddr.sin_addr.s_addr.to_le());
}
