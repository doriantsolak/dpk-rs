use std::{error::Error, io};

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

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
    ScoreRound,
}

#[derive(Debug)]
enum AppError {
    TooManyPlayers
}

#[derive(Clone)]
struct PlayerRoundInfo {
    won: bool,
    contra: bool,
    bids: u8,
    ex_ante: u8,
    fox: [String; 2],
    doppelkopf: u8,
    karlchen: bool,
    karlchen_caught: bool,
    round_score: u8,
    teammate: String,
}

impl PlayerRoundInfo {
    fn default() -> PlayerRoundInfo {
        PlayerRoundInfo {
            won: false,
            contra: false,
            bids: 0,
            ex_ante: 0,
            fox: [rand_string(), rand_string()],
            doppelkopf: 0,
            karlchen: false,
            karlchen_caught: false,
            round_score: 0,
            teammate: String::new(),
        }
    }
}

impl PlayerRoundInfo {
    fn increment_score(&mut self) {
        self.round_score += 1;
    }

    fn decrement_score(&mut self) {
        self.round_score -= 1;
    }

    fn score_player(&mut self) {
        // Award points points for a win
        // Award/substract points for potential bids
        match self.won {
            true => {
                self.increment_score();
                self.round_score += self.bids;
                self.round_score += self.ex_ante;
                match self.contra {
                    true => self.increment_score(),
                    false => (),
                };
            },
            false => {
                self.decrement_score();
                self.round_score -= self.bids;
                self.round_score -= self.ex_ante;
            },
        };
        // Award points for Doppelkopf, Karlchen, catching of Karlchen
        self.round_score += self.doppelkopf;
        match self.karlchen {
            true => self.increment_score(),
            false => (),
        };
        match self.karlchen {
            true => self.increment_score(),
            false => (),
        };
        match self.karlchen_caught {
            true => self.increment_score(),
            false => (),
        };
        // Award points for foxes
        match self.fox[0] {
            _ if self.fox[0] == self.teammate => self.increment_score(),
            _ => (),
        }
        match self.fox[0] {
            _ if self.fox[0] == self.teammate => self.increment_score(),
            _ => (),
        }
    }
}

#[derive(Clone)]
struct PlayerInfo {
    name: String,
    total_score: u64,
    past_scores: Vec<u64>,
    round_info: PlayerRoundInfo,
}

struct App {
    input_mode: InputMode,
    input: String,
    players: Vec<PlayerInfo>,
    player_list: StatefulList<String>,
}

impl App {
    fn new() -> App {
        App {
            input: String::new(),
            players: Vec::new(),
            player_list: StatefulList::with_items(vec![]),
            input_mode: InputMode::AddPlayer,
        }
    }

    fn add_player(&mut self) -> Result<(), AppError> {
        match self.players.len() {
            0..=3 => {
                self.players.push(PlayerInfo {
                    name: self.input.drain(..).collect(),
                    total_score: 0,
                    past_scores: Vec::new(),
                    round_info: PlayerRoundInfo::default(),
                })
            },
            _ => {
                return Err(AppError::TooManyPlayers)
            }
        }
        Ok(())
    }

    fn update_player_list(&mut self) {
        self.player_list = StatefulList::with_items(self.players.iter().cloned().map(|p|p.name).collect());
    }

    fn score_round(&mut self) {

    }

    fn set_input_mode_addplayer(&mut self) {
        self.input_mode = InputMode::AddPlayer;
    }

    fn set_input_mode_browse(&mut self) {
        self.input_mode = InputMode::Browse;
    }

    fn set_input_mode_scoreround(&mut self) {
        self.input_mode = InputMode::ScoreRound;
    }   

    fn add_event(&mut self) {}

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
                    KeyCode::Down => app.player_list.next(),
                    KeyCode::Up => app.player_list.previous(),
                    KeyCode::Char('a') => {
                        app.set_input_mode_addplayer();
                    }
                    KeyCode::Char('e') => {
                        app.add_event()
                    }
                    _ => {}
                },
                InputMode::AddPlayer => match key.code {
                    KeyCode::Enter => {
                        match app.add_player() {
                            Ok(_) => (),
                            Err(_) => app.set_input_mode_browse(),
                        }
                        app.update_player_list();
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
                InputMode::ScoreRound => match key.code {
                    _ => (),
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(25), Constraint::Percentage(25), Constraint::Percentage(25)].as_ref())
        .split(f.size());

    // let items: Vec<ListItem> = app
    //     .player_list
    //     .items
    //     .iter()
    //     .map(|i| ListItem::new(i.as_str()))
    //     .collect();

    // let items = List::new(items).
    //     block(Block::default().borders(Borders::ALL).title("Players"))
    //     .highlight_style(
    //         Style::default()
    //             .bg(Color::LightGreen)
    //             .add_modifier(Modifier::BOLD),
    //     ).highlight_symbol("> ");

    // f.render_stateful_widget(items, chunks[0], &mut app.player_list.state);

    if app.players.len() > 0 {
        render_player_blocks(f, &chunks, app);
    }

    match app.input_mode {
        InputMode::AddPlayer => { 
        let input = Paragraph::new(app.input.as_ref())
            .style(Style::default())
            .block(Block::default().borders(Borders::ALL).title("New player"));
        let area = centered_rect(60, 20, size);
        f.render_widget(Clear, area);
        f.render_widget(input, area);
        },
        InputMode::Browse => {},
        InputMode::ScoreRound => {},
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
        match self.state.selected() {
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

fn render_player_blocks<B: Backend>(f: &mut Frame<B>, chunks: &Vec<Rect>, app: &App) {

    for i in 0..app.players.len() {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(app.players[i].name.clone());
        f.render_widget(block, chunks[i])
    }
}

fn rand_string() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}