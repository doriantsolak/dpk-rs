use tui::{
    backend::Backend,
    Frame, 
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, Borders, Clear, Paragraph},
};
use super::app::{App,InputMode};
use super::render::*;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(1, 4),
            ]
            .as_ref(),
        )
        .split(size);

    if app.players.len() > 0 {
        render_player_blocks(f, chunks[0], app);
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
        InputMode::Browse => {
            
        }
    }
}
