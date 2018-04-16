
extern crate awl;
use std::env;
use awl::Awl;

fn main() {
    println!("awl is a tool to punch holes with.");

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);


    // let a = Awl {
    //     ip_addr: "44.00.32.14".to_string(),
    //     port: 1337
    // };

    // a.my_ip();
}
