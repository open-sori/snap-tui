use crossterm::event::{KeyCode, KeyEvent};
use crate::App;
use std::time::Duration;
use ratatui::{
    style::{Color, Style},
    text::Span,
};

pub async fn handle_input(app: &mut App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Char('q') => {
            app.should_quit = true;
        }
        KeyCode::Left => {
            if app.current_tab > 0 {
                app.current_tab -= 1;
                app.selected_item = None;
            }
        }
        KeyCode::Right => {
            if app.current_tab < 2 {
                app.current_tab += 1;
                app.selected_item = None;
            }
        }
        KeyCode::Char('g') => {
            // Switch to Groups tab
            app.current_tab = 0;
            app.selected_item = None;
        }
        KeyCode::Char('c') => {
            // Switch to Clients tab
            app.current_tab = 1;
                    app.selected_item = None;
                }
        KeyCode::Char('s') => {
            // Switch to Streams tab
            app.current_tab = 2;
            app.selected_item = None;
                    }
        KeyCode::Up => navigate_up(app),
        KeyCode::Down => navigate_down(app),
        KeyCode::Char('r') => {
            // Force a connection retry
            app.last_connection_attempt = std::time::Instant::now().checked_sub(Duration::from_secs(60)).unwrap_or_else(|| std::time::Instant::now());
            if let Err(e) = app.attempt_connection().await {
                app.error_message = Some(format!("Error refreshing Snapcast status: {}", e));
            } else {
                app.error_message = Some(
                    Span::styled(
                        "Successfully refreshed Snapcast status",
                        Style::default().fg(Color::Blue)
                    ).content.into()
                );
            }
            app.selected_item = None;
        }
        _ => {}
    }
    Ok(())
        }
fn navigate_up(app: &mut App) {
    if let Some(status) = &app.snapcast_client.status {
        match app.current_tab {
            0 => { // Groups tab
                if app.selected_item.is_none() {
                    app.selected_item = Some(0);
                } else if let Some(idx) = app.selected_item {
                    if idx > 0 {
                        app.selected_item = Some(idx - 1);
                    }
                }
            }
            1 => { // Clients tab
                let client_count = status.server.groups.iter()
                    .flat_map(|group| &group.clients)
                    .count();

                if client_count == 0 {
                    app.selected_item = None;
                    return;
                }

                if app.selected_item.is_none() {
                    app.selected_item = Some(0);
                } else if let Some(idx) = app.selected_item {
                    if idx > 0 {
                        app.selected_item = Some(idx - 1);
                    }
                }
            }
            2 => { // Streams tab
                if app.selected_item.is_none() {
                    app.selected_item = Some(0);
                } else if let Some(idx) = app.selected_item {
                    if idx > 0 {
                        app.selected_item = Some(idx - 1);
                    }
                }
            }
            _ => {}
        }
    }
}

fn navigate_down(app: &mut App) {
    if let Some(status) = &app.snapcast_client.status {
        match app.current_tab {
            0 => { // Groups tab
                let group_count = status.server.groups.len();
                if group_count == 0 {
                    app.selected_item = None;
                    return;
                }

                if app.selected_item.is_none() {
                    app.selected_item = Some(0);
                } else if let Some(idx) = app.selected_item {
                    if idx < group_count - 1 {
                        app.selected_item = Some(idx + 1);
                    }
                }
            }
            1 => { // Clients tab
                let client_count = status.server.groups.iter()
                    .flat_map(|group| &group.clients)
                    .count();

                if client_count == 0 {
                    app.selected_item = None;
                    return;
                }

                if app.selected_item.is_none() {
                    app.selected_item = Some(0);
                } else if let Some(idx) = app.selected_item {
                    if idx < client_count - 1 {
                        app.selected_item = Some(idx + 1);
                    }
                }
            }
            2 => { // Streams tab
                let stream_count = status.server.streams.len();
                if stream_count == 0 {
                    app.selected_item = None;
                    return;
                }

                if app.selected_item.is_none() {
                    app.selected_item = Some(0);
                } else if let Some(idx) = app.selected_item {
                    if idx < stream_count - 1 {
                        app.selected_item = Some(idx + 1);
                    }
                }
            }
            _ => {}
        }
    }
}
