mod header;
mod tabs;
mod streams;

pub mod clients;
pub mod groups;

use ratatui::Frame;
use crate::App;

pub use header::draw_header;
pub use tabs::draw_tabs;
pub use streams::draw_streams;

pub fn ui(f: &mut Frame, app: &App) {
    // Main vertical layout
    let main_layout = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Length(3),  // Header
            ratatui::layout::Constraint::Length(1),  // Tabs
            ratatui::layout::Constraint::Min(1),     // Content
        ])
        .split(f.size());

    // Draw header with server info
    draw_header(f, main_layout[0], app);

    // Draw tabs navigation
    draw_tabs(f, main_layout[1], app);

    // Draw content based on selected tab
    match app.current_tab {
        0 => draw_streams(f, main_layout[2], app),
        1 => draw_placeholder(f, main_layout[2], "Clients"),
        2 => draw_placeholder(f, main_layout[2], "Groups"),
        _ => draw_streams(f, main_layout[2], app),
    }
}

fn draw_placeholder(f: &mut Frame, area: ratatui::layout::Rect, title: &str) {
    let block = ratatui::widgets::Block::default()
        .title(title)
        .borders(ratatui::widgets::Borders::ALL);

    let paragraph = ratatui::widgets::Paragraph::new("This feature is not yet implemented")
        .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::NONE))
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(block, area);
    f.render_widget(paragraph, area);
}