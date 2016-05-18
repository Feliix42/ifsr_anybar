extern crate hyper;
extern crate core;

use hyper::Client;
use hyper::status::StatusCode;
use std::io::Read;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::env;
use core::str::FromStr;

mod anybar;


fn get_buerostatus() -> Option<bool> {
    let client = Client::new();
    let url = "https://www.ifsr.de/buerostatus/output.php";

    let mut res = match client.get(url).send() {
        Ok(resp) => resp,
        Err(_)   => return None,
    };

    // Check if response from Server is Status 200.
    if res.status != StatusCode::Ok {
        return None;
    }

    let mut onezero = String::new();
    res.read_to_string(&mut onezero).unwrap();

    println!("{}", onezero);
    match onezero.as_ref() {
        "0" => Some(false),
        "1" => Some(true),
        _   => {
            println!("Unknown return value from ifsr.de!");
            None
        }
    }
}


fn set_color(color: &str, port: u16) {
    let ip = Ipv4Addr::new(127, 0, 0, 1);

    // sender and target address
    let sender = SocketAddrV4::new(ip, 0);
    let target = SocketAddrV4::new(ip, port);

    // transform color into a vector
    let mut message: Vec<u8> = Vec::new();
    message.extend(color.as_bytes()
                          .iter()
                          .cloned());

    // send message
    anybar::send_message(SocketAddr::V4(sender),
                         SocketAddr::V4(target),
                         message)
}


fn main() {
    let color = match get_buerostatus() {
        Some(true)  => "green",
        Some(false) => "red",
        None        => "question",
    };
    println!("{}", color);

    let mut args = env::args();
    let ip = match u16::from_str(&args.nth(1).unwrap_or("1738".to_string())) {
        Ok(ip_addr) => ip_addr,
        Err(_)      => {
            println!("[Err] Could not parse port number.");
            return;
        }
    };

    set_color(color, ip);
}
