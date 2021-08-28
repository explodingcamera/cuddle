use std::{
    io::stdout,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use super::{ui, TabsState};
use crate::{Config, CuddleApp};
use anyhow::Result;
use crossterm::{
    cursor,
    event::{
        self, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use tui::{backend::CrosstermBackend, Terminal};

pub struct Dashboard<'a> {
    pub app: CuddleApp<'a>,
    pub config: Config,
    pub tabs: TabsState<'a>,
    pub title: &'a str,
    pub should_quit: bool,
}

impl<'a> Dashboard<'a> {
    pub fn new(app: CuddleApp<'a>) -> Result<Dashboard<'a>> {
        let config = (app.get_config)()?;
        Ok(Dashboard {
            app,
            title: "cuddle - code university dashboard",
            config: config,
            tabs: TabsState::new(vec!["Home", "Search", "Profile"]),
            should_quit: false,
        })
    }

    pub fn draw(mut self) -> Result<()> {
        let stdout = stdout();

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        enable_raw_mode()?;
        terminal.clear()?;

        loop {
            terminal.draw(|f| ui::draw(f, &mut self))?;

            match read()? {
                Event::Key(c) => self.on_key(c),
                _ => (),
            }

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    pub fn on_key(&mut self, e: KeyEvent) {
        match e {
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }
            | KeyEvent {
                code: KeyCode::Char('z'),
                modifiers: KeyModifiers::CONTROL,
            } => {
                self.should_quit = true;
            }
            _ => {}
        }
    }
}
