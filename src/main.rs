extern crate clap;
extern crate awl;

use std::env;
use clap::{Arg, App};
use awl::Awl;

fn main() {
    let matches = App::new("awl is a tool to punch holes with.")
            .version("0.1.0")
            .author("Jared Fowler <jared.a.fowler@gmail.com")
            .about("udp hole punching client and server")
            .arg(Arg::with_name("server")
                 .value_name("SERVER")
                 .help("Start a server listening for UDP connections")
                 .takes_value(true))
            .get_matches();

    let awl = Awl {
        ip_addr: String::from("0.0.0.1"),
        port: 15555
    };

    if matches.is_present("server") {
        let server_name = matches.value_of("server");
        println!("Server name is {:?}", server_name.unwrap());
    } else {
        println!("client");
    }
}
