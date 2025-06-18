mod list;
mod details;

use ratatui::Frame;
use crate::App;

pub use list::draw_streams_list;
pub use details::draw_stream_details;

pub fn draw_streams(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    // Horizontal layout for list and preview
    let content_layout = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            ratatui::layout::Constraint::Percentage(30),  // Streams list
            ratatui::layout::Constraint::Percentage(70),  // Stream details preview
        ])
        .split(area);

    // Draw the streams list
    draw_streams_list(f, content_layout[0], app);

    // Draw the stream details
    draw_stream_details(f, content_layout[1], app);
}