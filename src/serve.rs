extern crate libawl;

use std::net::{UdpSocket};

use libawl::{Client, Pool};

pub fn serve() -> () {
    let socket = UdpSocket::bind("127.0.0.1:3000")
        .expect("Could not connect on port 3000");
    let mut connection_pool = Pool::new();

    loop {
        let mut buf = [0; 16];
        let (amt, public_addr) = socket.recv_from(&mut buf).unwrap();
        if amt != 10 {
            continue;
        }

        let new_client = Client::new(&buf, public_addr).unwrap();

        match connection_pool.find_client(&new_client.target) {
            Some(c) => {
                socket.connect(&new_client.public).unwrap();
                socket.send(&c.serialize()).unwrap();
                socket.connect(&c.public).unwrap();
                socket.send(&new_client.serialize()).unwrap();
            },
            None => (
                match connection_pool.client_in_pool(&new_client.private.ip()) {
                    Some(_) => println!("Client already pending connection"),
                    None => connection_pool.connections.push(new_client),
                }
            ),
        }
    }
}
