use ratatui::{
    layout::Rect,
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use ratatui::layout::Alignment;
use crate::App;

pub fn draw_clients_list(f: &mut Frame, area: Rect, app: &App) {
    let list_area = area;
    let inner_list_area = Rect {
        x: list_area.x + 1,
        y: list_area.y + 2,
        width: list_area.width.saturating_sub(2),
        height: list_area.height.saturating_sub(2),
    };

    // Calculate clients count for title
    let clients_count = if let Some(status) = &app.snapcast_client.status {
        status.server.groups.iter()
            .flat_map(|group| &group.clients)
            .count()
    } else {
        0
    };

    // Create a centered title for the clients list
    let title = format!("Clients ({})", clients_count);
    let title_span = Span::styled(
        title,
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    );

    let title_line = Line::from(vec![title_span]);
    let title_paragraph = Paragraph::new(title_line)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));

    if let Some(status) = &app.snapcast_client.status {
        // Collect all clients from all groups
        let all_clients: Vec<_> = status.server.groups
            .iter()
            .flat_map(|group| group.clients.iter())
            .collect();

        let mut list_state = ListState::default();
        list_state.select(app.selected_item);

        let items: Vec<ListItem> = all_clients
            .iter()
            .enumerate()
            .map(|(idx, client)| {
                let prefix = if Some(idx) == app.selected_item {
                    "> "  // Selection indicator
                } else {
                    "  "  // Regular indentation
                };

                // Format client information - just showing the client ID
                let client_info = format!("{}{}", prefix, client.id);

                let content = Line::from(vec![
                    Span::styled(
                        client_info,
                        if Some(idx) == app.selected_item {
                            Style::default()
                                .fg(Color::Green)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::White)
                        }
                    ),
                ]);

                ListItem::new(content)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::NONE));

        // Render the centered title above the list
        f.render_widget(title_paragraph, list_area);
        f.render_stateful_widget(list, inner_list_area, &mut list_state);
    } else {
        let content = Paragraph::new("No data available. Press 'r' to refresh.")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::NONE));

        f.render_widget(title_paragraph, list_area);
        f.render_widget(content, inner_list_area);
    }
}