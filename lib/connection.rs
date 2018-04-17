extern crate byteorder;

use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::error::Error as StdError;

use byteorder::{NetworkEndian, ByteOrder};

#[derive(Debug)]
pub struct Client {
    pub private: SocketAddr,
    pub public: SocketAddr,
}

impl Client {
    fn make(msg: &[u8; 6], public: SocketAddr) -> Result<Client, Box<StdError>> {
        let private = Client::parse_private_connection(msg);
        Ok(Client{
            private,
            public,
        })
    }
}

impl Client {
    pub fn parse_private_connection(msg: &[u8]) -> SocketAddr {
        let ip = Ipv4Addr::new(msg[0], msg[1], msg[2], msg[3]);
        let port = NetworkEndian::read_u16(&msg[4..6]);
        SocketAddr::new(IpAddr::V4(ip), port)
    }
}

pub struct Pool {
    pub connections: Vec<Client>,
}

impl Pool {
    pub fn client_in_pool(&self, target_ip: &IpAddr) -> Option<usize> {
        for (i, ref client) in self.connections.iter().enumerate() {
            if client.private.ip() == *target_ip {
                return Some(i);
            }
        }
        None
    }

    pub fn find_client(&mut self, target_ip: &IpAddr) -> Option<Client> {
        match self.client_in_pool(&target_ip) {
            Some(pos) => return Some(self.connections.swap_remove(pos)),
            _ => return None,
        }
    }
}

pub fn parse_target_ip(msg: &[u8; 4]) -> IpAddr {
    IpAddr::V4(Ipv4Addr::new(msg[0], msg[1], msg[2], msg[3]))
}
