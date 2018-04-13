extern crate byteorder;

use std::net::{UdpSocket, SocketAddr, IpAddr, Ipv4Addr};
use byteorder::{NetworkEndian, ByteOrder};

static SERVER_IP: &'static [u8] = &[127, 0, 0, 1];
const SERVER_PORT: u16 = 3000;

static PEER_IP: &'static [u8] = &[233, 241, 0, 0];

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:3400").unwrap();
    let dest = SocketAddr::new(
        IpAddr::V4(
            Ipv4Addr::new(SERVER_IP[0],
                          SERVER_IP[1],
                          SERVER_IP[2],
                          SERVER_IP[3])
            ),SERVER_PORT
    );
    socket.connect(&dest).expect("Failed to connect to rendevous server");

    let mut payload = vec!(127, 0, 0, 1);

    let mut port = [0; 2];
    NetworkEndian::write_u16(&mut port, 3400);
    payload.extend(port.iter().cloned());
    payload.extend(PEER_IP.iter().cloned());

    socket.send(&payload).unwrap();
}
