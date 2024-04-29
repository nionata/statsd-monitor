use crate::parser;
use crate::server::UdpServer;
use std::collections::HashMap;
use std::str::from_utf8;

/// Get a group of measurements.
pub trait MeasurementsProvider {
    fn update_measurements(&mut self, measurements: &mut HashMap<String, f64>);
}

pub struct StatsdMeasurementsProvider {
    udp_server: UdpServer,
}

impl StatsdMeasurementsProvider {
    pub fn new(udp_server: UdpServer) -> Self {
        Self { udp_server }
    }
}

impl MeasurementsProvider for StatsdMeasurementsProvider {
    fn update_measurements(&mut self, measurements: &mut HashMap<String, f64>) {
        let packet = self.udp_server.try_get().unwrap();

        let lines = from_utf8(packet).unwrap();

        let statsd_measurements = parser::from_statsd_line(lines);

        for m in statsd_measurements {
            measurements.insert(m.name, m.value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{net::UdpSocket, thread};

    #[test]
    fn publish() {
        const PORT: u16 = 10000;
        const BUF_SIZE: u16 = 256;
        const A_NAME: &str = "metric.b";
        const B_NAME: &str = "metric.a";
        const PACKETS: u8 = 10;

        let addr = || format!("{}:{}", "0.0.0.0", PORT);
        let make_line = |val: f64| format!("{}:{}|g\n{}:{}|g\n", A_NAME, val, B_NAME, val);

        let server_handle = thread::spawn(move || {
            let server = UdpServer::new(&addr(), BUF_SIZE).unwrap();

            let mut provider = StatsdMeasurementsProvider::new(server);

            let mut measurements: HashMap<String, f64> = HashMap::new();

            for i in 0..PACKETS {
                provider.update_measurements(&mut measurements);

                let i = i as f64;

                assert_eq!(measurements.len(), 2);
                assert_eq!(measurements.get(&A_NAME.to_string()).unwrap(), &i);
                assert_eq!(measurements.get(&B_NAME.to_string()).unwrap(), &i);
            }
        });

        let client_handle = thread::spawn(move || {
            let client = UdpSocket::bind("0.0.0.0:0").unwrap();

            for i in 0..PACKETS {
                assert_eq!(
                    client
                        .send_to(make_line(i as f64).as_bytes(), addr())
                        .unwrap(),
                    26,
                );
            }
        });

        client_handle.join().unwrap();
        server_handle.join().unwrap();
    }
}
