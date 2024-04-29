use std::io;
use std::net::UdpSocket;

pub struct UdpServer {
    socket: UdpSocket,
    buf: Vec<u8>,
}

impl UdpServer {
    pub fn new(addr: &str, buffer_size: u16) -> io::Result<Self> {
        let socket = UdpSocket::bind(addr)?;

        Ok(Self {
            socket,
            buf: vec![0; buffer_size.into()],
        })
    }

    /// Try and get the next udp packet.
    ///
    /// # Blocking
    ///
    /// This function will block until a packet is available or there is an error.
    pub fn try_get(&mut self) -> io::Result<&[u8]> {
        match self.socket.recv_from(&mut self.buf) {
            Ok((size, _)) => Ok(&self.buf[0..size]),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn publish() {
        const PORT: u16 = 10000;
        const BUF_SIZE: u16 = 256;
        const PACKET_SIZE_BYTES: usize = 10;
        const PACKETS: u8 = 10;
        const PACKET: [u8; PACKET_SIZE_BYTES] = [0; PACKET_SIZE_BYTES];

        let addr = || format!("{}:{}", "0.0.0.0", PORT);

        let server_handle = thread::spawn(move || {
            let mut server = UdpServer::new(&addr(), BUF_SIZE).unwrap();
            for _ in 0..PACKETS {
                assert_eq!(server.try_get().unwrap(), PACKET);
            }
        });

        let client_handle = thread::spawn(move || {
            let client = UdpSocket::bind("0.0.0.0:0").unwrap();

            for _ in 0..PACKETS {
                assert_eq!(client.send_to(&PACKET, addr()).unwrap(), PACKET_SIZE_BYTES,);
            }
        });

        server_handle.join().unwrap();
        client_handle.join().unwrap();
    }
}
