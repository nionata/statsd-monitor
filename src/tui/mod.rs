use crate::provider::MeasurementsProvider;
use crossterm::{
    event::{self, poll, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::{Bar, BarChart, BarGroup, Block},
    Frame,
};
use std::{
    collections::HashMap,
    io::{self, Stdout},
    thread,
    time::Duration,
};

pub struct Tui {
    measurements: HashMap<String, f64>,
    exited: bool,
}

impl Tui {
    pub fn new() -> Self {
        Self {
            measurements: HashMap::new(),
            exited: false,
        }
    }

    pub fn run(&mut self, provider: &mut impl MeasurementsProvider) -> io::Result<()> {
        let mut terminal = Self::init()?;

        while !self.exited {
            thread::sleep(Duration::from_millis(10));

            provider.update_measurements(&mut self.measurements);

            terminal.draw(|frame| self.draw_frame(frame))?;

            if Self::is_event_available()? {
                self.handle_events()?;
            }
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

        // TODO encapsulate this better...

        let mut bars = Vec::new();

        let mut max_val = 1.0;
        let mut max_width = 1;

        for (k, v) in self.measurements.iter() {
            bars.push(
                Bar::default()
                    .value(*v as u64)
                    .text_value(v.to_string())
                    .label(k.clone().into()),
            );

            if k.len() > max_width {
                max_width = k.len();
            }

            if *v > max_val {
                max_val = *v;
            }
        }

        let data = BarGroup::default().bars(&bars);

        frame.render_widget(
            BarChart::default()
                .data(data)
                .bar_width(max_width.try_into().unwrap())
                .max(max_val as u64)
                .block(block),
            frame.size(),
        )
    }

    fn is_event_available() -> io::Result<bool> {
        poll(Duration::from_millis(10))
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
