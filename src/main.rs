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
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Char('r') => {
            app.snapcast_client.fetch_status().await?;
        }
        KeyCode::Left | KeyCode::Right => {
            // Switch tabs
            let tab_count = 3; // We have 3 tabs
            if key.code == KeyCode::Left {
                app.current_tab = if app.current_tab > 0 {
                    app.current_tab - 1
                } else {
                    tab_count - 1
                };
            } else {
                app.current_tab = (app.current_tab + 1) % tab_count;
            }
        }
        KeyCode::Down => {
            if let Some(status) = &app.snapcast_client.status {
                let item_count = match app.current_tab {
                    0 => status.server.streams.len(),
                    _ => 0,
                };

                if item_count > 0 {
                    app.selected_item = Some(match app.selected_item {
                        Some(selected) => {
                            if selected + 1 < item_count {
                                selected + 1
                            } else {
                                0
                            }
                        }
                        None => 0,
                    });
                }
            }
        }
        KeyCode::Up => {
            if let Some(status) = &app.snapcast_client.status {
                let item_count = match app.current_tab {
                    0 => status.server.streams.len(),
                    _ => 0,
                };

                if item_count > 0 {
                    app.selected_item = Some(match app.selected_item {
                        Some(selected) => {
                            if selected > 0 {
                                selected - 1
                            } else {
                                item_count - 1
                            }
                        }
                        None => 0,
                    });
                }
            }
        }
        _ => {}
    }
    Ok(())
}