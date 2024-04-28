mod tui;
use tui::Tui;

mod server;

fn main() {
    Tui::new()
        .expect("Failed to init terminal")
        .run()
        .expect("Failed to run terminal");
}
