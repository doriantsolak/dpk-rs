use std::io;

use tui::{backend::Backend, Terminal};

use crossterm::event::{self, Event, KeyCode};

use super::app::{App, InputMode};
use super::ui::ui;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        
        terminal.draw(|f| ui(f, &mut app))?;
        
        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::AddPlayer => match key.code {
                    KeyCode::Enter => app.add_player(),
                    _ => (),
                }
                _ => (),
            }
        }
    }
}
