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

use crate::server::StatsdServer;

pub struct Tui {
    exited: bool,
}

impl Tui {
    pub fn new() -> io::Result<Self> {
        Ok(Self { exited: false })
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut server = StatsdServer::new()?;

        let mut terminal = Self::init()?;

        while !self.exited {
            thread::sleep(Duration::from_millis(10));

            let _val = server.try_get();

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
                .data(&[("metric", 10)])
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
