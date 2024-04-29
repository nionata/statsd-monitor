use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::{BarChart, Block},
    Frame,
};
use std::{
    io::{self, Stdout},
    thread,
    time::Duration,
};

pub struct Tui {
    // TODO: make this a trait so we can mock it out
    // server: StatsdServer,
    exited: bool,
}

impl Tui {
    // pub fn new(server: StatsdServer) -> Self {
    pub fn new() -> Self {
        Self {
            // server,
            exited: false,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut terminal = Self::init()?;

        while !self.exited {
            thread::sleep(Duration::from_millis(10));

            terminal.draw(|frame| self.draw_frame(frame))?;

            self.handle_events()?;
        }

        Ok(())
    }

    fn init() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
        io::stdout().execute(EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;

        Terminal::new(CrosstermBackend::new(io::stdout()))
    }

    fn draw_frame(&mut self, frame: &mut Frame) {
        let block = Block::bordered().title("Statsd Monitor");
        frame.render_widget(
            BarChart::default()
                .data(&[("Metric", 10)])
                .bar_width(10)
                .max(100)
                .block(block),
            frame.size(),
        )
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => self.handle_key_press(event),
            _ => Ok(()),
        }
    }

    fn handle_key_press(&mut self, event: KeyEvent) -> io::Result<()> {
        match event.code {
            KeyCode::Char('q') | KeyCode::Esc => self.exit(),
            KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) => self.exit(),
            _ => Ok(()),
        }
    }

    fn exit(&mut self) -> io::Result<()> {
        io::stdout().execute(LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;

        self.exited = true;
        Ok(())
    }
}
