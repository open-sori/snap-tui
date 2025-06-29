mod header;
mod tabs;
mod streams;
mod clients;
mod groups;
mod footer; // Add this line

use ratatui::Frame;
use crate::App;

pub use header::draw_header;
pub use tabs::draw_tabs;
pub use streams::draw_streams;
pub use clients::draw_clients;
pub use groups::draw_groups;
pub use footer::draw_footer; // Add this line

pub fn ui(f: &mut Frame, app: &App) {
    // Main vertical layout
    let main_layout = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Length(3),  // Header
            ratatui::layout::Constraint::Length(1),  // Tabs
            ratatui::layout::Constraint::Min(1),     // Content
            ratatui::layout::Constraint::Length(3),  // Footer for error messages (increased height)
        ])
        .split(f.size());

    // Draw header with server info
    draw_header(f, main_layout[0], app);

    // Draw tabs navigation
    draw_tabs(f, main_layout[1], app);

    // Draw content based on selected tab
    match app.current_tab {
        0 => draw_groups(f, main_layout[2], app),
        1 => draw_clients(f, main_layout[2], app),
        2 => draw_streams(f, main_layout[2], app),
        _ => draw_groups(f, main_layout[2], app),
    }

    // Draw footer with error messages
    draw_footer(f, main_layout[3], app);
}