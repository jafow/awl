extern crate byteorder;

use std::net::{SocketAddr, Ipv4Addr, SocketAddrV4};
use std::error::Error as StdError;

use byteorder::{NetworkEndian, ByteOrder};

fn spread_values_over_buffer(dest: &mut[u8], src: &[u8], start: usize) {
    for (i, value) in src.iter().enumerate() {
        dest[i + start] = *value;
    }
}

#[derive(Debug)]
pub struct Client {
    pub private: SocketAddrV4,
    pub public: SocketAddrV4,
    pub target: Ipv4Addr,
}

impl Client {
    pub fn new(msg: &[u8], public: SocketAddr) -> Result<Self, Box<StdError>> {
        let private = Client::parse_private_connection(msg);
        let target = Client::parse_target_ip(&msg[6..]);

        let public = match public {
            SocketAddr::V4(addr) => addr,
            _ => panic!("Woah there that's not IPv4"),
        };

        Ok(Client{
            private,
            public,
            target,
        })
    }

    pub fn parse_private_connection(msg: &[u8]) -> SocketAddrV4 {
        let ip = Ipv4Addr::new(msg[0], msg[1], msg[2], msg[3]);
        let port = NetworkEndian::read_u16(&msg[4..6]);
        SocketAddrV4::new(ip, port)
    }

    fn parse_target_ip(msg: &[u8]) -> Ipv4Addr {
        Ipv4Addr::new(msg[0], msg[1], msg[2], msg[3])
    }
}

impl Client {
    // Serializes a Client struct into a array with the following values:
    // Target Private IP: 4 bytes
    // Target Private Port: 2 Bytes
    // Target Public IP: 4 bytes
    // Target Public Port: 2 Bytes
    pub fn serialize(&self) -> [u8;12] {
        let mut msg = [0; 12];
        for (i, value) in self.private.ip().octets().iter().enumerate() {
            msg[i] = *value;
        }
        spread_values_over_buffer(&mut msg, &self.private.ip().octets(), 0);
        NetworkEndian::write_u16(&mut msg[4..6], self.private.port());
        spread_values_over_buffer(&mut msg, &self.public.ip().octets(), 6);
        NetworkEndian::write_u16(&mut msg[10..], self.public.port());
        msg
    }
}

pub struct Pool {
    pub connections: Vec<Client>,
}

impl Pool {
    pub fn new() -> Pool {
        Pool {
            connections: Vec::new(),
        }
    }
}

impl Pool {
    pub fn client_in_pool(&self, target_ip: &Ipv4Addr) -> Option<usize> {
        for (i, ref client) in self.connections.iter().enumerate() {
            if client.private.ip() == target_ip {
                return Some(i);
            }
        }
        None
    }

    pub fn find_client(&mut self, target_ip: &Ipv4Addr) -> Option<Client> {
        match self.client_in_pool(&target_ip) {
            Some(pos) => return Some(self.connections.swap_remove(pos)),
            _ => return None,
        }
    }
}
