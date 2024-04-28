use std::io;
use std::time::Duration;
use std::{net::UdpSocket, str::from_utf8};

pub struct StatsdServer {
    socket: UdpSocket,
    buf: Vec<u8>,
}

impl StatsdServer {
    pub fn new(addr: &str, buffer_size: u16) -> io::Result<Self> {
        let socket = UdpSocket::bind(addr)?;
        socket.set_read_timeout(Some(Duration::from_secs(1)))?;

        Ok(Self {
            socket,
            buf: vec![0; buffer_size.into()],
        })
    }

    pub fn try_get(&mut self) -> Option<&str> {
        match self.socket.recv_from(&mut self.buf) {
            Ok((size, _)) => from_utf8(&self.buf[0..size]).ok(),
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use metrics_exporter_statsd::StatsdBuilder;

    use super::*;

    #[test]
    fn publish() {
        const METRIC: &str = "metric.test";
        const PORT: u16 = 10000;
        const BUF_SIZE: u16 = 256;

        thread::spawn(|| {
            let recorder = StatsdBuilder::from("0.0.0.0", PORT)
                .with_buffer_size(BUF_SIZE.into())
                .build(Some(""))
                .expect("Failed to init statsd");

            metrics::set_global_recorder(recorder).expect("Failed to init recorder");

            let mut i = 0;
            let tick_gauage = metrics::gauge!(METRIC);

            loop {
                thread::sleep(Duration::from_millis(1));

                i += 1;

                tick_gauage.set(i);
            }
        });

        let mut server = StatsdServer::new(&format!("0.0.0.0:{}", PORT), BUF_SIZE).unwrap();
        loop {
            println!("{:?}", server.try_get().unwrap());
        }
    }
}
