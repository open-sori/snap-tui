use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, List, ListItem, ListState, Tabs},
    Frame,
};
use crate::{App};

pub fn ui(f: &mut Frame, app: &App) {
    // Main vertical layout - removed the footer constraint
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Length(1),  // Tabs
            Constraint::Min(1),     // Content
        ])
        .split(f.size());

    // Draw header with server info
    draw_header(f, main_layout[0], app);

    // Draw tabs navigation
    draw_tabs(f, main_layout[1], app);

    // Always show streams list with details panel
    draw_streams_list(f, main_layout[2], app);
}

fn draw_header(f: &mut Frame, area: Rect, app: &App) {
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
        .alignment(ratatui::layout::Alignment::Center)
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
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::NONE));

        f.render_widget(server_info, info_area);
    }
}

fn draw_tabs(f: &mut Frame, area: Rect, _app: &App) {
    let tabs = vec!["Streams", "Clients", "Groups"];

    let titles = tabs.iter().map(|t| {
        let (first, rest) = t.split_at(1);
        Line::from(vec![
            Span::styled(first, Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
            Span::styled(rest, Style::default().fg(Color::White)),
        ])
    }).collect::<Vec<_>>();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .select(0);

    f.render_widget(tabs, area);
}

fn draw_streams_list(f: &mut Frame, area: Rect, app: &App) {
    // Horizontal layout for list and preview
    let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),  // Streams list
            Constraint::Percentage(70),  // Stream details preview
        ])
        .split(area);

    // Left side - Streams list
    let list_area = content_layout[0];
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
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::NONE));

    // Create a block with centered title
    let block = Block::default()
        .borders(Borders::ALL);

    if let Some(status) = &app.snapcast_client.status {
        let mut list_state = ListState::default();
        if let Some(selected) = app.selected_stream {
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

                if Some(idx) == app.selected_stream {
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

    // Right side - Preview with full details
    let preview_area = content_layout[1];
    let inner_preview_area = Rect {
        x: preview_area.x + 1,
        y: preview_area.y + 1,
        width: preview_area.width - 2,
        height: preview_area.height - 2,
    };

    let preview_block = Block::default()
        .title(" Stream Details ")
        .borders(Borders::ALL);

    let preview_content = if let (Some(status), Some(selected_idx)) = (&app.snapcast_client.status, app.selected_stream) {
        if let Some(stream) = status.server.streams.get(selected_idx) {
            let mut lines = Vec::new();

            // Add all stream details to preview
            lines.push(Line::from(vec![
                Span::styled("Name:", Style::default().fg(Color::Yellow)),
                Span::styled(&stream.uri.query.name, Style::default().fg(Color::White)),
            ]));

            lines.push(Line::from(vec![
                Span::styled("Status:", Style::default().fg(Color::Yellow)),
                Span::styled(&stream.status, Style::default().fg(Color::White)),
            ]));

            lines.push(Line::from(vec![
                Span::styled("Path:", Style::default().fg(Color::Yellow)),
                Span::styled(&stream.uri.path, Style::default().fg(Color::White)),
            ]));

            lines.push(Line::from(vec![
                Span::styled("Scheme:", Style::default().fg(Color::Yellow)),
                Span::styled(&stream.uri.scheme, Style::default().fg(Color::White)),
            ]));

            if let Some(chunk_ms) = &stream.uri.query.chunk_ms {
                lines.push(Line::from(vec![
                    Span::styled("Chunk MS:", Style::default().fg(Color::Yellow)),
                    Span::styled(chunk_ms, Style::default().fg(Color::White)),
                ]));
            }

            if let Some(codec) = &stream.uri.query.codec {
                lines.push(Line::from(vec![
                    Span::styled("Codec:", Style::default().fg(Color::Yellow)),
                    Span::styled(codec, Style::default().fg(Color::White)),
                ]));
            }

            if let Some(mode) = &stream.uri.query.mode {
                lines.push(Line::from(vec![
                    Span::styled("Mode:", Style::default().fg(Color::Yellow)),
                    Span::styled(mode, Style::default().fg(Color::White)),
                ]));
            }

            if let Some(sample_format) = &stream.uri.query.sample_format {
                lines.push(Line::from(vec![
                    Span::styled("Sample Format:", Style::default().fg(Color::Yellow)),
                    Span::styled(sample_format, Style::default().fg(Color::White)),
                ]));
            }

            Paragraph::new(lines)
        } else {
            Paragraph::new("Select a stream to see details")
        }
    } else {
        Paragraph::new("No stream selected")
    };

    f.render_widget(preview_block, preview_area);
    f.render_widget(preview_content, inner_preview_area);
}