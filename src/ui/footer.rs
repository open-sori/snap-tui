use ratatui::{
    layout::{Rect, Margin},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::App;

pub fn draw_footer(f: &mut Frame, area: Rect, app: &App) {
    // Create a block for the footer panel
    let block = Block::default()
        .title(" Infos ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));

    // Create an inner area with proper margins
    let inner_area = Rect {
        x: area.x + 2,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };

    // Create the content for the footer
    let content = if let Some(message) = &app.error_message {
        // Error message content (red)
        Paragraph::new(message.clone())
            .style(Style::default().fg(Color::Red))
            .block(Block::default())
    } else if let Some(message) = &app.info_message {
        // Info message content (blue)
        Paragraph::new(message.clone())
            .style(Style::default().fg(Color::Blue))
            .block(Block::default())
    } else {
        // Get connection status
        let status_text = if app.connection_error.is_some() {
            Span::styled("Disconnected", Style::default().fg(Color::Red))
        } else if app.snapcast_client.status.is_some() {
            Span::styled("Connected", Style::default().fg(Color::Green))
        } else {
            Span::styled("Connecting...", Style::default().fg(Color::Yellow))
        };

        // Help message content with purple for regular messages
        Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(Color::Gray)),
                status_text,
            ]),
            Line::from(vec![
                Span::styled("Press ", Style::default().fg(Color::Magenta)),
                Span::styled("'q'", Style::default().fg(Color::Yellow)),
                Span::styled(" to quit | ", Style::default().fg(Color::Magenta)),
                Span::styled("'r'", Style::default().fg(Color::Yellow)),
                Span::styled(" to refresh | ", Style::default().fg(Color::Magenta)),
                Span::styled("←→", Style::default().fg(Color::Yellow)),
                Span::styled(" to switch tabs", Style::default().fg(Color::Magenta)),
            ]),
            Line::from(vec![
                Span::styled("Connected to: ", Style::default().fg(Color::Magenta)),
                Span::styled(
                    format!("{}", app.snapcast_client.get_url()),
                    Style::default().fg(Color::White)
                ),
            ]),
        ])
        .block(Block::default())
    };

    // Render the footer
    f.render_widget(block, area);
    f.render_widget(content, inner_area);
}