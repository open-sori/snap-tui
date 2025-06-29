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
        x: area.x + 4,
        y: area.y + 2,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };

    // Check if we have data and a selected item
    if let Some(status) = &app.snapcast_client.status {
        if let Some(selected_idx) = app.selected_item {
            if let Some(stream) = status.server.streams.get(selected_idx) {
                let mut details = Vec::new();

                details.push(Line::from(vec![
                    Span::styled("Id: ", Style::default().fg(Color::Yellow)),
                    Span::styled(stream.id.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Name: ", Style::default().fg(Color::Yellow)),
                    Span::styled(stream.uri.query.name.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Status: ", Style::default().fg(Color::Yellow)),
                    Span::styled(stream.status.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Host: ", Style::default().fg(Color::Yellow)),
                    Span::styled(stream.uri.host.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Scheme: ", Style::default().fg(Color::Yellow)),
                    Span::styled(stream.uri.scheme.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Path: ", Style::default().fg(Color::Yellow)),
                    Span::styled(stream.uri.path.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Mode: ", Style::default().fg(Color::Yellow)),
                    Span::styled(stream.uri.query.mode.clone().expect("REASON").to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Codec: ", Style::default().fg(Color::Yellow)),
                    Span::styled(stream.uri.query.codec.clone().expect("REASON").to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Sample Format: ", Style::default().fg(Color::Yellow)),
                    Span::styled(stream.uri.query.sample_format.clone().expect("REASON").to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Chunk Ms: ", Style::default().fg(Color::Yellow)),
                    Span::styled(stream.uri.query.chunk_ms.clone().expect("REASON").to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Fragment: ", Style::default().fg(Color::Yellow)),
                    Span::styled(stream.uri.fragment.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Raw: ", Style::default().fg(Color::Yellow)),
                    Span::styled(stream.uri.raw.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Can Control: ", Style::default().fg(Color::Yellow)),
                    Span::styled(stream.properties.can_control.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Can Play: ", Style::default().fg(Color::Yellow)),
                    Span::styled(stream.properties.can_play.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Can Pause: ", Style::default().fg(Color::Yellow)),
                    Span::styled(stream.properties.can_pause.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Can Seek: ", Style::default().fg(Color::Yellow)),
                    Span::styled(stream.properties.can_seek.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Can Go Next: ", Style::default().fg(Color::Yellow)),
                    Span::styled(stream.properties.can_go_next.to_string(), Style::default().fg(Color::White)),
                ]));

                details.push(Line::from(vec![
                    Span::styled("Can Go Previous: ", Style::default().fg(Color::Yellow)),
                    Span::styled(stream.properties.can_go_previous.to_string(), Style::default().fg(Color::White)),
                ]));

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