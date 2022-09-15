use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    backend::Backend,
    Frame,
    widgets::{Block,Borders},
};
use super::app::App;

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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

pub fn render_current_selection<B: Backend>(f: &mut Frame<B>, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL);
    f.render_widget(block, area)
}

pub fn render_player_scores<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL);
    f.render_widget(block, area)
}

pub fn render_events<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL);
    f.render_widget(block, area)
}

pub fn render_log<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL);
    f.render_widget(block, area)
}