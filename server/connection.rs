use std::net::{SocketAddr, IpAddr};

#[derive(Debug)]
pub struct Client {
    pub private: SocketAddr,
    pub public: SocketAddr,
}

impl Client {
    pub fn serialize_found_peer(&self) {
        // returns u8 array of destination peer's public IP 
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
