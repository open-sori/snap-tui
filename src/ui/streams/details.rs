use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph}
};
use crate::App;

pub fn draw_stream_details(f: &mut Frame, area: Rect, app: &App) {
    let preview_area = area;
    let inner_preview_area = Rect {
        x: preview_area.x + 1,
        y: preview_area.y + 1,
        width: preview_area.width - 2,
        height: preview_area.height - 2,
    };

    let preview_block = Block::default()
        .title(" Stream Details ")
        .borders(Borders::ALL);

    let preview_content = if let (Some(status), Some(selected_idx)) = (&app.snapcast_client.status, app.selected_item) {
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