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
    let title = "snap-tui";

    let status = if app.connection_error.is_some() {
        Span::styled(
            "Disconnected",
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD)
        )
    } else if app.snapcast_client.status.is_some() {
        Span::styled(
            "Connected",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        )
    } else {
        Span::styled(
            "Connecting...",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        )
    };

    let title_line = Line::from(vec![
        Span::styled(
            title,
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        ),
        Span::styled(" | ", Style::default().fg(Color::White)),
        status
    ]);

    let title_paragraph = Paragraph::new(title_line)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));

    f.render_widget(title_paragraph, area);

    // Server info below the title - only show if connected
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
    } else if let Some(error) = &app.connection_error {
        let error_message = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Error: ", Style::default().fg(Color::Red)),
                Span::styled(error, Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("Press 'r' to retry connection", Style::default().fg(Color::Yellow)),
            ])
        ])
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));

        f.render_widget(error_message, info_area);
    } else {
        let connecting_message = Paragraph::new("Connecting to server...")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::NONE));

        f.render_widget(connecting_message, info_area);
    }
}