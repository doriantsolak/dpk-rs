use std::{error::Error, io};

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

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
    SelectPlayer,
    SelectGameEvent,
}

enum GameEvent {
    Bid,
    Fox,
    Doppelkopf,
    Karlchen,
    KarlchenCaught,
    None,
}

enum Bid {
    Regular,
    No90,
    No60,
    No30,
    Black,
}


#[derive(Debug)]
enum AppError {
    TooManyPlayers,
}

struct Round {
    // time:
    counter: u8,
    player_round_info: [PlayerRoundInfo; 4],
}

impl Round {
    fn default() -> Round {
        Round {
            counter: 0,
            player_round_info: [
                PlayerRoundInfo::default(),
                PlayerRoundInfo::default(),
                PlayerRoundInfo::default(),
                PlayerRoundInfo::default(),
            ],
        }
    }
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
            }
            false => {
                self.decrement_score();
                self.round_score -= self.bids;
                self.round_score -= self.ex_ante;
            }
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
}

struct App {
    input_mode: InputMode,
    input: String,
    players: Vec<PlayerInfo>,
    rounds: Vec<PlayerRoundInfo>,
    player_list: StatefulList<String>,
    game_event_list: StatefulList<&'static str>,
    main_menu_list: StatefulList<&'static str>,
    player_round_info_list: Vec<StatefulList<&'static str>>,
    current_game_event: GameEvent,
    current_player_event: String,
}

impl App {
    fn new() -> App {
        App {
            input: String::new(),
            players: Vec::new(),
            player_list: StatefulList::with_items(vec![]),
            game_event_list: StatefulList::with_items(
                vec!["Bid", "Doppelkopf", "Fox", "Karlchen", "Karlchen caught"]),
            input_mode: InputMode::AddPlayer,
            rounds: Vec::new(),
            current_game_event: GameEvent::None,
            current_player_event: String::new(),
            main_menu_list: StatefulList::with_items(
                vec!["New round", "Score round", "Export data (NYI)", "Exit"]
            ),
            player_round_info_list: vec![StatefulList::with_items(vec![]); 4],
        }
    }

    fn add_player(&mut self) -> Result<(), AppError> {
        match self.players.len() {
            0..=3 => self.players.push(PlayerInfo {
                name: self.input.drain(..).collect(),
                total_score: 0,
            }),
            _ => return Err(AppError::TooManyPlayers),
        }
        Ok(())
    }

    fn update_player_list(&mut self) {
        self.player_list = StatefulList::with_items(self.players.clone().into_iter().map(|p|p.name).collect());
    }

    fn score_round(&mut self) {}

    fn set_input_mode_addplayer(&mut self) {
        self.input_mode = InputMode::AddPlayer;
    }

    fn set_input_mode_browse(&mut self) {
        self.input_mode = InputMode::Browse;
    }

    fn set_input_mode_selectplayer(&mut self) {
        self.input_mode = InputMode::SelectPlayer;
    }

    fn set_input_mode_selectgameevent(&mut self) {
        self.input_mode = InputMode::SelectGameEvent;
    }

    fn new_round(&mut self) {

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
                InputMode::SelectPlayer => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down => app.player_list.next(),
                    KeyCode::Up => app.player_list.previous(),
                    KeyCode::Char('a') => {
                        app.set_input_mode_addplayer();
                    }
                    KeyCode::Enter => {
                        app.set_input_mode_selectgameevent();
                        app.current_player_event = app.player_list.current_item();
                    },
                    
                    _ => {}
                },
                InputMode::SelectGameEvent => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down => app.game_event_list.next(),
                    KeyCode::Up => app.game_event_list.previous(),
                    KeyCode::Enter => {
                        app.current_game_event = match app.game_event_list.items[app.game_event_list.state.selected().unwrap()] {
                            "Bid" => GameEvent::Bid,
                            "Doppelkopf" => GameEvent::Doppelkopf,
                            "Fox" => GameEvent::Fox,
                            "Karlchen" => GameEvent::Karlchen,
                            "Karlchen caught" => GameEvent::KarlchenCaught,
                            _ => GameEvent::None,
                        };
                        // app.player_round_info_list[
                        //     app.player_list.state.selected().unwrap()
                        // ].items.push(
                        //     match app.current_game_event {
                        //         Bid => (),
                        //         Doppelkopf => (),
                        //         Fox => (),
                        //         Karlchen => (),
                        //         KarlchenCaught => (),
                        //     }
                        // )
                    }
                    _ => (),
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
                InputMode::Browse => match key.code {
                    KeyCode::Enter => match app.main_menu_list.current_item() {
                        "New round" => app.set_input_mode_selectplayer(),
                        "Score round" => app.score_round(),
                        "Export data (NYI)" => (),
                        "Exit" => return Ok(()),
                        _ => (),
                    },
                    KeyCode::Down => app.main_menu_list.next(),
                    KeyCode::Up => app.main_menu_list.previous(),
                    _ => (),
                },
                InputMode::SelectGameEvent => match key.code {
                    _ => (),
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    let chunks = Layout::default()
        //.direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(1, 4),
                Constraint::Ratio(3, 4),
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
            ]
            .as_ref(),
        )
        .split(size);
    //.split(f.size());

    if app.players.len() > 0 {
        render_player_blocks(f, chunks[0], app);
    }

    if app.players.len() == 4 {
        render_round(f, chunks[1], app)
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
        InputMode::SelectPlayer => {},
        InputMode::SelectGameEvent => {},
    }
}

#[derive(Clone)]
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
        self.state.select(Some(i));
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
        self.state.select(Some(i));
    }

    fn current_item(&self) -> T
    where T: Clone 
    {
        self.items[self.state.selected().unwrap()].clone()
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

fn render_player_blocks<B: Backend>(f: &mut Frame<B>, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
        ].as_ref(),)
        .split(area);

    for i in 0..app.players.len() {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(app.players[i].name.clone());
        f.render_widget(block, chunks[i])
    }
}

fn render_main_menu<B: Backend>(f: &mut Frame<B>, area: Rect, app: &mut App) {

    let items: Vec<ListItem> = app.main_menu_list.items.clone().into_iter().map(|i| ListItem::new(i)).collect();

    let items = List::new(items).
        block(Block::default().borders(Borders::ALL).title("Menu"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        ).highlight_symbol("> ");

    f.render_stateful_widget(items, area, &mut app.main_menu_list.state);
}


fn render_player_selection<B: Backend>(f: &mut Frame<B>, area: Rect, app: &mut App) {

    let items: Vec<ListItem> = app.player_list.items.clone().into_iter().map(|i| ListItem::new(i)).collect();

    let items = List::new(items).
        block(Block::default().borders(Borders::ALL).title("Players"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        ).highlight_symbol("> ");

    f.render_stateful_widget(items, area, &mut app.player_list.state);
}

fn render_game_event_selection<B: Backend>(f: &mut Frame<B>, area: Rect, app: &mut App) {

    let items: Vec<ListItem> = app.game_event_list.items.clone().into_iter().map(|i| ListItem::new(i)).collect();

    let items = List::new(items).
        block(Block::default().borders(Borders::ALL).title("Game events"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        ).highlight_symbol("> ");

    f.render_stateful_widget(items, area, &mut app.game_event_list.state);
}



fn render_round<B: Backend>(f: &mut Frame<B>, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4)
        ].as_ref(),)
        .split(area);

        render_main_menu(f, chunks[0], app);
        render_player_selection(f, chunks[1], app);
        render_game_event_selection(f, chunks[2], app);
        // TODO: render_round_log(f, chunks[4], app);

        ()
}

fn rand_string() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}

// fn log_message() -> 