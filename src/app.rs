use tui::text::{Text};

pub enum InputMode {
    AddPlayer,
    SelectPlayer,
    SelectEvent,
}

pub enum Event {
    Bid,
    Fox,
    Doppelkopf,
    Re,
    Won,
    Karlchen,
    KarlchenCaught,
}

pub enum DpkError {
}

pub struct Player {
    name: String,
    score: u32,
}

pub struct App {
    pub input_mode: InputMode,
    pub input: String,
    pub players: Vec<Player>,
}

impl App {
    pub fn new() -> App {
        App {
            input_mode: InputMode::AddPlayer,
            input: String::new(),
            players: Vec::new(),
        }
    }

    pub fn add_player(&mut self) -> () {
        self.players.push(Player {
            name: self.input.drain(..).collect(),
            score: 0,
        });
    }

}
