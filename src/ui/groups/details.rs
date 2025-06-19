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
        x: area.x + 4,
        y: area.y + 2,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };

    // Check if we have data and a selected item
    if let Some(status) = &app.snapcast_client.status {
        if let Some(selected_idx) = app.selected_item {
            if let Some(group) = status.server.groups.get(selected_idx) {
                let mut details = Vec::new();

                details.push(Line::from(vec![
                    Span::styled("Id: ", Style::default().fg(Color::Yellow)),
                    Span::styled(group.id.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Name: ", Style::default().fg(Color::Yellow)),
                    Span::styled(group.name.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Stream Id: ", Style::default().fg(Color::Yellow)),
                    Span::styled(group.stream_id.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Muted: ", Style::default().fg(Color::Yellow)),
                    Span::styled(group.muted.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Clients: ", Style::default().fg(Color::Yellow)),
                ]));

                // Add each client's ID
                if group.clients.is_empty() {
                    details.push(Line::from(vec![
                        Span::styled("  No clients connected", Style::default().fg(Color::Gray)),
                    ]));
                } else {
                    for client in &group.clients {
                        details.push(Line::from(vec![
                            Span::styled("  - Id: ", Style::default().fg(Color::Blue)),
                            Span::styled(client.id.to_string(), Style::default().fg(Color::White)),
                        ]));

                        details.push(Line::from(vec![
                            Span::styled("    Connected: ", Style::default().fg(Color::Blue)),
                            Span::styled(client.connected.to_string(), Style::default().fg(Color::White)),
                        ]));
                    }
                }

                // Create the paragraph with all the details
                let details = Paragraph::new(details);

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