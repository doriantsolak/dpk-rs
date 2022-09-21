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
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 5),
                Constraint::Ratio(4, 5),
            ]
            .as_ref(),
        )
        .split(size);

    let upper_block = &chunks[0];
    let lower_block = &chunks[1];

    let lower_blocks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(2, 4),
        ].as_ref(),)
        .split(*lower_block);

    render_current_selection(f, *upper_block, app);

    render_player_scores(f,  &app.players, lower_blocks[0]);

    render_event_selection(f, lower_blocks[1]);

    render_log(f, lower_blocks[2]);

    match app.input_mode {
        InputMode::AddPlayer => {
            let input = Paragraph::new(app.input.clone())
                .style(Style::default())
                .block(Block::default().borders(Borders::ALL).title("New player"));
            let area = centered_rect(40, 20, chunks[1]);
            f.render_widget(Clear, area);
            f.render_widget(input, area);
        },
        InputMode::SelectEvent => {

        },
        InputMode::SelectPlayer => {

        }
    }
}
