use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Corner, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};

enum InputMode {
    Browse,
    AddPlayer,
}

#[derive(Clone)]
struct PlayerInfo {
    name: String,
    score: u64,
}

struct App<'a> {
    input_mode: InputMode,
    input: String,
    players: Vec<PlayerInfo>,
    items: StatefulList<&'a str>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            input: String::new(),
            players: Vec::new(),
            items: StatefulList::with_items(vec![]),
            input_mode: InputMode::Browse,
        }
    }

    fn add_player(&mut self, name: String) {
        self.players.push(PlayerInfo {
            name: name,
            score: 0,
        })
    }

    fn trigger_user_input(&mut self) {
        self.input_mode = InputMode::AddPlayer;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Browse => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down => app.items.next(),
                    KeyCode::Up => app.items.previous(),
                    KeyCode::Char('a') => {
                        app.trigger_user_input();
                    }
                    _ => {}
                },
                InputMode::AddPlayer => match key.code {
                    KeyCode::Enter => {
                        app.players.push(PlayerInfo {
                            name: app.input.drain(..).collect(),
                            score: 0,
                        });
                        // app.items.push(app.players.last().unwrap().name.as_str()); // This line is giving me trouble.
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Browse;
                    }
                    _ => {}
                },
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|&i| ListItem::new(i))
        .collect();

    let items = List::new(items).
        block(Block::default().borders(Borders::ALL).title("Players"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        ).highlight_symbol("> ");

    f.render_stateful_widget(items, chunks[0], &mut app.items.state);

    match app.input_mode {
        InputMode::AddPlayer => { 
        let input = Paragraph::new(app.input.as_ref())
            .style(Style::default())
            .block(Block::default().borders(Borders::ALL).title("New player"));
        let area = centered_rect(60, 20, size);
        f.render_widget(Clear, area);
        f.render_widget(input, area);
        }
        InputMode::Browse => {}
    }



}

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.items.len(),
        };
    }

    fn push(&mut self, new_item: T) {
        self.items.push(new_item)
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}