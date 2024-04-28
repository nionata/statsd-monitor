mod parser;
mod server;
mod tui;

use server::StatsdServer;
use tui::Tui;

fn main() {
    // TODO: make addr a cli arg
    let statsd_server =
        StatsdServer::new("0.0.0.0:8125", 2048).expect("Failed to initialize statd server");

    Tui::new(statsd_server)
        .run()
        .expect("Failed to run terminal");
}
