use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::App;

pub fn draw_group_details(f: &mut Frame, area: Rect, app: &App) {
    // Create a block with a title for the group details
    let block = Block::default()
        .title(" Group Details ")
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
            if let Some(group) = status.server.groups.get(selected_idx) {
                // Create a paragraph with the group details
                let details = Paragraph::new(vec![
                    Line::from(vec![
                        Span::styled("ID: ", Style::default().fg(Color::Yellow)),
                        Span::styled(&group.id, Style::default().fg(Color::White)),
                    ]),
                    Line::from(vec![
                        Span::styled("Stream ID: ", Style::default().fg(Color::Yellow)),
                        Span::styled(&group.stream_id, Style::default().fg(Color::White)),
                    ]),
                    Line::from(vec![
                        Span::styled("Muted: ", Style::default().fg(Color::Yellow)),
                        Span::styled(format!("{}", group.muted), Style::default().fg(Color::White)),
                    ]),
                    Line::from(vec![
                        Span::styled("Clients: ", Style::default().fg(Color::Yellow)),
                        Span::styled(format!("{}", group.clients.len()), Style::default().fg(Color::White)),
                    ]),
                ]);

                // Render the details
                f.render_widget(details, inner_area);
                return;
            }
        }
    }

    // If no selection or no data, show a message
    let paragraph = Paragraph::new("Select a group to see details")
        .style(Style::default().fg(Color::White))
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(paragraph, inner_area);
}