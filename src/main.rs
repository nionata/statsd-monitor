mod parser;
mod server;
mod tui;

use server::UdpServer;
use tui::Tui;

fn main() {
    // TODO: make addr a cli arg
    let udp_server =
        UdpServer::new("0.0.0.0:8125", 2048).expect("Failed to initialize statd server");

    Tui::new().run().expect("Failed to run terminal");
}
