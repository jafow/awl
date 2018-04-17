extern crate libawl;

use std::net::{UdpSocket};

use libawl::{Client, Pool};

pub fn serve() -> () {
    let socket = UdpSocket::bind("127.0.0.1:3000").expect("Could not connect on port 80");
    let mut connection_pool = Pool::new();

    loop {
        let mut buf = [0; 16];
        let (amt, public_addr) = socket.recv_from(&mut buf).unwrap();
        if amt != 10 {
            continue;
        }

        let private_addr = Client::parse_private_connection(&buf);
        let target_peer = &buf[6..];
        let target_peer = libawl::parse_target_ip(target_peer);

        let new_client = Client {
            private: private_addr,
            public: public_addr,
        };

        match connection_pool.find_client(&target_peer) {
            Some(c) => println!("Found them! {:?}", c),
            None => (
                match connection_pool.client_in_pool(&new_client.private.ip()) {
                    Some(_) => println!("Client already pending connection"),
                    None => connection_pool.connections.push(new_client),
                }
            ),
        }
    }
}
