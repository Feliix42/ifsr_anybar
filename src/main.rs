extern crate hyper;

use hyper::Client;
use hyper::status::StatusCode;
use std::io::Read;

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

fn main() {
    match get_buerostatus() {
        Some(is_open) => println!("Is it open? {}", is_open),
        None          => println!("Didn't work! :o"),
    };
}
