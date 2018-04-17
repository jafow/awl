extern crate byteorder;

use std::str::FromStr;
use std::net::{UdpSocket, SocketAddr};

use byteorder::{NetworkEndian, ByteOrder};

static SERVER: &'static str = "127.0.0.1:3000";
static PEER_IP: &'static [u8] = &[233, 241, 0, 0];

pub fn connect() {
    let socket = UdpSocket::bind("127.0.0.1:3400").unwrap();
    let dest = SocketAddr::from_str(SERVER).unwrap();
    socket.connect(&dest).expect("Failed to connect to rendevous server");

    let mut port = [0; 2];
    NetworkEndian::write_u16(&mut port, 3400);

    let mut payload = vec!(127, 0, 0, 1);
    payload.extend(port.iter().cloned());
    payload.extend(PEER_IP.iter().cloned());

    socket.send(&payload).unwrap();
}
