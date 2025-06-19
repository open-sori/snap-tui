use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::App;

pub fn draw_client_details(f: &mut Frame, area: Rect, app: &App) {
    // Create a block with a title for the client details
    let block = Block::default()
        .title(" Client Details ")
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
            // Collect all clients from all groups
            let all_clients: Vec<_> = status.server.groups
                .iter()
                .flat_map(|group| group.clients.iter())
                .collect();

            if let Some(client) = all_clients.get(selected_idx) {
                let mut details = Vec::new();

                details.push(Line::from(vec![
                    Span::styled("Id: ", Style::default().fg(Color::Yellow)),
                    Span::styled(client.id.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Instance: ", Style::default().fg(Color::Yellow)),
                    Span::styled(client.config.instance.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Name: ", Style::default().fg(Color::Yellow)),
                    Span::styled(client.config.name.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Version: ", Style::default().fg(Color::Yellow)),
                    Span::styled(client.snapclient.version.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Connected: ", Style::default().fg(Color::Yellow)),
                    Span::styled(client.connected.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Ip: ", Style::default().fg(Color::Yellow)),
                    Span::styled(client.host.ip.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Mac: ", Style::default().fg(Color::Yellow)),
                    Span::styled(client.host.mac.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Latency: ", Style::default().fg(Color::Yellow)),
                    Span::styled(client.config.latency.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Volume: ", Style::default().fg(Color::Yellow)),
                    Span::styled(client.config.volume.percent.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Muted: ", Style::default().fg(Color::Yellow)),
                    Span::styled(client.config.volume.muted.to_string(), Style::default().fg(Color::White)),
                ]));

                // Create the paragraph with all the details
                let details = Paragraph::new(details);

                // Render the details
                f.render_widget(details, inner_area);
                return;
            }
        }
    }

    // If we have data but nothing is selected, show an empty block
    let empty_block = Block::default().borders(Borders::NONE);
    f.render_widget(empty_block, inner_area);
}