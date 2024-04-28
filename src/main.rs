mod tui;
use tui::Tui;

fn main() {
    Tui::new()
        .expect("Failed to init terminal")
        .run()
        .expect("Failed to run terminal");
}
