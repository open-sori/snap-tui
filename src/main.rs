mod snapcast;
mod ui;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io;

#[derive(Debug)]
struct App {
    should_quit: bool,
    snapcast_client: snapcast::SnapcastClient,
    selected_item: Option<usize>,
    current_tab: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App {
        should_quit: false,
        snapcast_client: snapcast::SnapcastClient::new("ws://localhost:1780/jsonrpc".to_string()),
        selected_item: None,
        current_tab: 0,
    };

    // Initial data fetch
    app.snapcast_client.fetch_status().await?;

    // Main loop
    while !app.should_quit {
        terminal.draw(|f| ui::ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            handle_input(&mut app, key).await?;
        }
    }

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

async fn handle_input(app: &mut App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Char('q') => {
            app.should_quit = true;
        }
        KeyCode::Char('h') | KeyCode::Left => {
            // Handle tab navigation
            if app.current_tab > 0 {
                app.current_tab -= 1;
                app.selected_item = None; // Reset selection when changing tabs
            }
        }
        KeyCode::Char('l') | KeyCode::Right => {
            // Handle tab navigation
            if app.current_tab < 2 { // Assuming we have 3 tabs (0, 1, 2)
                app.current_tab += 1;
                app.selected_item = None; // Reset selection when changing tabs
            }
        }
        KeyCode::Up => {
            // Handle list navigation
            if let Some(status) = &app.snapcast_client.status {
                match app.current_tab {
                    0 => { // Streams tab
                        if let Some(selected) = app.selected_item {
                            if selected > 0 {
                                app.selected_item = Some(selected - 1);
                            }
                        } else if !status.server.streams.is_empty() {
                            app.selected_item = Some(0);
                        }
                    }
                    1 => { // Clients tab
                        // Implement client navigation if needed
                    }
                    2 => { // Groups tab
                        if let Some(selected) = app.selected_item {
                            if selected > 0 {
                                app.selected_item = Some(selected - 1);
                            }
                        } else if !status.server.groups.is_empty() {
                            app.selected_item = Some(0);
                        }
                    }
                    _ => {}
                }
            }
        }
        KeyCode::Down => {
            // Handle list navigation
            if let Some(status) = &app.snapcast_client.status {
                match app.current_tab {
                    0 => { // Streams tab
                        if let Some(selected) = app.selected_item {
                            if selected + 1 < status.server.streams.len() {
                                app.selected_item = Some(selected + 1);
                            }
                        } else if !status.server.streams.is_empty() {
                            app.selected_item = Some(0);
                        }
                    }
                    1 => { // Clients tab
                        // Implement client navigation if needed
                    }
                    2 => { // Groups tab
                        if let Some(selected) = app.selected_item {
                            if selected + 1 < status.server.groups.len() {
                                app.selected_item = Some(selected + 1);
                            }
                        } else if !status.server.groups.is_empty() {
                            app.selected_item = Some(0);
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
    Ok(())
}
