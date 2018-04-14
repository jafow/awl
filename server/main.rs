extern crate byteorder;

use std::net::{SocketAddr, UdpSocket, IpAddr, Ipv4Addr};
use byteorder::{NetworkEndian, ByteOrder};

mod connection;

fn parse_private_connection(msg: &[u8]) -> SocketAddr {
    let ip = Ipv4Addr::new(msg[0], msg[1], msg[2], msg[3]);
    let port = NetworkEndian::read_u16(&msg[4..6]);
    SocketAddr::new(IpAddr::V4(ip), port)
}

fn connect(){
    let socket = UdpSocket::bind("127.0.0.1:3000").expect("Could not connect on port 80");
    let mut connection_pool = connection::Pool {
        connections: Vec::new(),
    };

    loop {
        let mut buf = [0; 16];
        let (amt, public_addr) = socket.recv_from(&mut buf).unwrap();
        if amt != 10 {
            continue;
        }

        let private_addr = parse_private_connection(&buf);
        let target_peer = &buf[6..];
        let target_peer = IpAddr::V4(Ipv4Addr::new(target_peer[0],
                                        target_peer[1],
                                        target_peer[2],
                                        target_peer[3]));

        let new_client = connection::Client {
            private: private_addr,
            public: public_addr,
        };

        match connection_pool.find_client(&target_peer) {
            Some(c) => (
                println!("Found them! {:?}", c);
                socket.send(&c.serialize_found_peer())
                .unwrap().expect("Failed to send target peer to client");
            ),
            None => (
                match connection_pool.client_in_pool(&new_client.private.ip()) {
                    Some(_) => println!("Client already pending connection"),
                    None => connection_pool.connections.push(new_client),
                }
            ),
        }
    }
}

fn main() {
    connect();
}
