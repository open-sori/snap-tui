use ratatui::{
    Frame,
    layout::{Rect, Alignment},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Paragraph, Block, Borders}
};
use crate::App;

pub fn draw_header(f: &mut Frame, area: Rect, app: &App) {
    // Create a centered title with connection status
    let title = if app.snapcast_client.status.is_some() {
        "snap-tui (connected)"
    } else {
        "snap-tui (disconnected)"
    };

    let title_span = Span::styled(
        title,
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
    );

    let title_line = Line::from(vec![title_span]);
    let title_paragraph = Paragraph::new(title_line)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));

    f.render_widget(title_paragraph, area);

    // Server info below the title
    let info_area = Rect {
        x: area.x,
        y: area.y + 1,
        width: area.width,
        height: area.height - 1,
    };

    if let Some(status) = &app.snapcast_client.status {
        let server = &status.server.server.snapserver;
        let server_info = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Server: ", Style::default().fg(Color::Yellow)),
                Span::styled(&server.name, Style::default().fg(Color::White)),
                Span::styled(" | ", Style::default().fg(Color::Yellow)),
                Span::styled("Version: ", Style::default().fg(Color::Yellow)),
                Span::styled(&server.version, Style::default().fg(Color::White)),
            ])
        ])
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));

        f.render_widget(server_info, info_area);
    }
}