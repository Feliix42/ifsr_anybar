// The contents of this source file are taken from https://github.com/urschrei/rust_anybar
// See license.txt for license information.

use std::net;

fn socket(listen_on: net::SocketAddr) -> net::UdpSocket {
    let attempt = net::UdpSocket::bind(listen_on);
    let socket;
    match attempt {
        Ok(sock) => {
            socket = sock;
        }
        Err(err) => panic!("Could not bind: {}", err),
    }
    socket
}


pub fn send_message(send_addr: net::SocketAddr, target: net::SocketAddr, data: Vec<u8>) {
    let socket = socket(send_addr);
    let _ = socket.send_to(&data, target);
    drop(socket);
}
