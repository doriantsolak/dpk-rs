pub enum InputMode {
    AddPlayer,
    Browse,
}

pub enum AppError {
}

pub struct Player {
    pub name: String,
    pub score: u64,
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
