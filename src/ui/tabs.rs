use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Tabs},
    Frame,
};
use crate::App;

pub fn draw_tabs(f: &mut Frame, area: Rect, app: &App) {
    let tabs = vec!["Groups", "Clients", "Streams"];

    let titles = tabs.iter().enumerate().map(|(i, t)| {
        let (first, rest) = t.split_at(1);
        let is_selected = i == app.current_tab;

        Line::from(vec![
            Span::styled(
                first,
                Style::default()
                    .fg(if is_selected { Color::Green } else { Color::Yellow })
                    .add_modifier(Modifier::UNDERLINED)
            ),
            Span::styled(
                rest,
                Style::default()
                    .fg(if is_selected { Color::Green } else { Color::White })
                    .add_modifier(if is_selected { Modifier::BOLD } else { Modifier::empty() })
            ),
        ])
    }).collect::<Vec<_>>();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .select(app.current_tab);

    f.render_widget(tabs, area);
}