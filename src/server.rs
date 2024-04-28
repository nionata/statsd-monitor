use std::io::{self, Read};
use std::time::Duration;
use std::{net::UdpSocket, str::from_utf8};

pub struct StatsdServer {
    socket: UdpSocket,
}

impl StatsdServer {
    pub fn new() -> io::Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:8125")?;
        socket.set_read_timeout(Some(Duration::from_secs(1)))?;

        Ok(Self { socket })
    }

    pub fn try_get(&mut self) -> Option<usize> {
        let mut buf: [u8; 16] = [0; 16];

        match self.socket.recv_from(&mut buf) {
            Ok((size, addr)) => {
                println!("{} {}", size, addr);
                Some(size)
            }
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }
}
