mod parser;
mod provider;
mod server;
mod tui;

use provider::StatsdMeasurementsProvider;
use server::UdpServer;
use tui::Tui;

fn main() {
    // TODO: make addr a cli arg
    let udp_server =
        UdpServer::new("0.0.0.0:8125", 2048).expect("Failed to initialize statd server");

    let mut provider = StatsdMeasurementsProvider::new(udp_server);

    Tui::new()
        .run(&mut provider)
        .expect("Failed to run terminal");
}
