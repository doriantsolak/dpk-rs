use tui::text::{Text};
use strum::{IntoStaticStr};

pub enum InputMode {
    AddPlayer,
    Browse,
}

#[derive(IntoStaticStr)]
pub enum Event<'a> {
    Fox(&'a str),
    Won,
    Re,
    Bid,
    Doppelkopf,
    Karlchen,
    Karlchen_caught,
}

pub enum DpkError {
}

pub struct Player {
    pub name: String,
    pub score: u64,
}



impl From<&Player> for Text<'_> {
    fn from(player: &Player) -> Self {
        Text::raw(vec![player.name.clone(), player.score.clone().to_string()].join(": "))
    }
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
