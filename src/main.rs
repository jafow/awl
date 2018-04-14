extern crate awl;

use awl::Awl;

fn main() {
    println!("awl is a tool to punch holes with.");
    let a = Awl {
        ip_addr: "44.00.32.14".to_string(),
        port: 1337
    };

    a.my_ip();
}
