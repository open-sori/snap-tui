mod list;
mod details;

use ratatui::Frame;
use crate::App;

pub use list::draw_clients_list;
pub use details::draw_client_details;

pub fn draw_clients(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    // Horizontal layout for list and details
    let content_layout = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            ratatui::layout::Constraint::Percentage(30),  // Clients list
            ratatui::layout::Constraint::Percentage(70),  // Client details
        ])
        .split(area);

    // Draw the clients list
    draw_clients_list(f, content_layout[0], app);

    // Draw the client details
    draw_client_details(f, content_layout[1], app);
}