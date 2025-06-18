mod list;
mod details;

use ratatui::Frame;
use crate::App;

pub use list::draw_groups_list;
pub use details::draw_group_details;

pub fn draw_groups(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    // Horizontal layout for list and details
    let content_layout = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            ratatui::layout::Constraint::Percentage(30),  // Groups list
            ratatui::layout::Constraint::Percentage(70),  // Group details
        ])
        .split(area);

    // Draw the groups list
    draw_groups_list(f, content_layout[0], app);

    // Draw the group details
    draw_group_details(f, content_layout[1], app);
}