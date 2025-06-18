use ratatui::{
    layout::Rect,
    widgets::{Block, Borders},
    Frame,
};
use crate::App;

pub fn draw_client_details(f: &mut Frame, area: Rect, _app: &App) {
    // Create a block with a title but no content, with spaces around the title
    let block = Block::default()
        .title(" Client Details ")
        .borders(Borders::ALL);

    f.render_widget(block, area);
}