extern crate anybar;
extern crate buerostatus;

use anybar::Color;
use anybar::Anybar;

use std::str::FromStr;
use std::env;


fn main() {
    let color = match buerostatus::get_buerostatus() {
        Ok(true)  => Color::Green,     // someone is in the office
        Ok(false) => Color::Red,       // no one there
        Err(_)    => Color::Question,  // could not fetch the status
    };

    // take the UDP port of Anybar as command line argument or default to port 1738
    let mut args = env::args();
    let port = match u16::from_str(&args.nth(1).unwrap_or("1738".to_string())) {
        Ok(ip_addr) => ip_addr,
        Err(_)      => {
            println!("[Err] Could not parse port number.");
            return;
        }
    };

    let mut bar = Anybar::new(port);
    bar.set_color(color).unwrap();
}
