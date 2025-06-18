use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::App;

pub fn draw_stream_details(f: &mut Frame, area: Rect, app: &App) {
    // Create a block with a title for the stream details
    let block = Block::default()
        .title(" Stream Details ")
        .borders(Borders::ALL);

    // Render the block
    f.render_widget(block, area);

    // Create an inner area with margins
    let inner_area = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };

    // Check if we have data and a selected item
    if let Some(status) = &app.snapcast_client.status {
        if let Some(selected_idx) = app.selected_item {
        if let Some(stream) = status.server.streams.get(selected_idx) {
                // Create a paragraph with the stream details
                let details = Paragraph::new(vec![
                    Line::from(vec![
                Span::styled("ID: ", Style::default().fg(Color::Yellow)),
                Span::styled(&stream.id, Style::default().fg(Color::White)),
                    ]),
                    Line::from(vec![
                Span::styled("Status: ", Style::default().fg(Color::Yellow)),
                Span::styled(&stream.status, Style::default().fg(Color::White)),
                    ]),
                    Line::from(vec![
                Span::styled("URI: ", Style::default().fg(Color::Yellow)),
                Span::styled(&stream.uri.raw, Style::default().fg(Color::White)),
                    ]),
                    Line::from(vec![
                        Span::styled("Path: ", Style::default().fg(Color::Yellow)),
                        Span::styled(&stream.uri.path, Style::default().fg(Color::White)),
                    ]),
                ]);

                // Render the details
                f.render_widget(details, inner_area);
                return;
            }
        }
        // If we have data but nothing is selected, show an empty block
        let empty_block = Block::default().borders(Borders::NONE);
        f.render_widget(empty_block, inner_area);
        } else {
        // If no data is available, show a message
        let paragraph = Paragraph::new("No data available. Press 'r' to refresh.")
            .style(Style::default().fg(Color::White))
            .alignment(ratatui::layout::Alignment::Center)
            .block(Block::default().borders(Borders::NONE));

        f.render_widget(paragraph, inner_area);
        }
}
