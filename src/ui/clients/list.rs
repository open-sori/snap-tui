use ratatui::{
    layout::Rect,
    style::{Style, Color, Modifier},
    widgets::{Block, Borders, Paragraph},
    text::{Span, Line},
    Frame,
};
use crate::App;

pub fn draw_clients_list(f: &mut Frame, area: Rect, _app: &App) {
    // Create a centered title
    let title = Span::styled(
        "Clients",
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    );

    let title_line = Line::from(vec![title]);
    let title_paragraph = Paragraph::new(title_line)
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::NONE));

    // Render just the centered title without any borders
    f.render_widget(title_paragraph, area);
}