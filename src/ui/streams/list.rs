use ratatui::{
    Frame,
    layout::{Rect, Alignment},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, List, ListItem, ListState}
};
use crate::App;

pub fn draw_streams_list(f: &mut Frame, area: Rect, app: &App) {
    let list_area = area;
    let inner_list_area = Rect {
        x: list_area.x + 1,
        y: list_area.y + 1,
        width: list_area.width - 2,
        height: list_area.height - 2,
    };

    // Calculate streams count for title
    let streams_count = if let Some(status) = &app.snapcast_client.status {
        status.server.streams.len()
    } else {
        0
    };

    // Create a centered title for the streams list
    let title = format!("Streams ({})", streams_count);
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
        let mut list_state = ListState::default();
        if let Some(selected) = app.selected_item {
            list_state.select(Some(selected));
        }

        let items: Vec<ListItem> = status.server.streams
            .iter()
            .enumerate()
            .map(|(idx, stream)| {
                let name = &stream.uri.query.name;
                let content = Line::from(vec![
                    Span::styled(name, Style::default().fg(Color::White)),
                ]);

                if Some(idx) == app.selected_item {
                    ListItem::new(content)
                        .style(Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD)
                            .add_modifier(Modifier::REVERSED))
                } else {
                    ListItem::new(content)
                }
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::NONE))
            .highlight_style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            )
            .highlight_symbol("> ");

        // Render the centered title above the list
        f.render_widget(title_paragraph, list_area);
        f.render_stateful_widget(list, inner_list_area, &mut list_state);
    } else {
        let content = Paragraph::new("No data available. Press 'r' to refresh.")
            .block(Block::default().borders(Borders::NONE));

        f.render_widget(title_paragraph, list_area);
        f.render_widget(content, inner_list_area);
    }
}