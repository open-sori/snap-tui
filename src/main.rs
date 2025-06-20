pub mod input;
pub mod snapcast;
pub mod ui;

use crossterm::{
    event::{self, Event, KeyCode, poll},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::{
    env,
    io,
    time::Duration,
};
use clap::{Arg, Command};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "1780";
const RETRY_INTERVAL: Duration = Duration::from_secs(5);

#[derive(Debug)]
pub struct App {
    pub should_quit: bool,
    pub snapcast_client: snapcast::SnapcastClient,
    pub selected_item: Option<usize>,
    pub current_tab: usize,
    pub connection_error: Option<String>,
    pub last_connection_attempt: std::time::Instant,
    pub error_message: Option<String>,
    pub info_message: Option<String>, // Add this line for non-error messages
}

impl App {
    pub async fn attempt_connection(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.last_connection_attempt = std::time::Instant::now();
        match self.snapcast_client.fetch_status().await {
        Ok(_) => {
                self.connection_error = None;
    Ok(())
}
            Err(e) => {
                let error_msg = format!("Connection error: {}", e);
                self.connection_error = Some(error_msg.clone());
                self.error_message = Some(error_msg);
                Err(e)
            }
        }
    }
}

fn get_snapserver_host() -> String {
    env::var("SNAPSERVER_HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string())
}

fn get_snapserver_port() -> String {
    env::var("SNAPSERVER_PORT").unwrap_or_else(|_| DEFAULT_PORT.to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let matches = Command::new("snap-tui")
        .about("A TUI for Snapcast")
        .arg(
            Arg::new("version")
                .long("version")
                .action(clap::ArgAction::SetTrue)
                .help("Prints version information")
        )
        .arg(
            Arg::new("host")
                .long("host")
                .value_name("HOST")
                .help("Sets the Snapcast server host")
                .default_value(DEFAULT_HOST),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .value_name("PORT")
                .help("Sets the Snapcast server port")
                .default_value(DEFAULT_PORT),
        )
        .get_matches();

    // Check for version flag
    if matches.get_flag("version") {
        println!("Version: v{}", VERSION);
        return Ok(());
    }

    // Get host and port with proper precedence
    let host = matches.get_one::<String>("host")
        .map(|s| s.clone())
        .unwrap_or_else(|| get_snapserver_host());

    let port = matches.get_one::<String>("port")
        .map(|s| s.clone())
        .unwrap_or_else(|| get_snapserver_port());

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state with configured host and port
    let server_url = format!("ws://{}:{}/jsonrpc", host, port);
    let mut app = App {
        should_quit: false,
        snapcast_client: snapcast::SnapcastClient::new(server_url),
        selected_item: None,
        current_tab: 0,
        connection_error: None,
        last_connection_attempt: std::time::Instant::now(),
        error_message: None,
        info_message: None, // Add this line for non-error messages
    };

    // Initial data fetch attempt
    app.attempt_connection().await?;

    // Main loop
    while !app.should_quit {
        // Draw UI
        terminal.draw(|f| ui::ui(f, &app))?;

        // Handle input events with timeout to prevent blocking
        if poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Always allow quitting with Ctrl+C or q
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q') {
                    app.should_quit = true;
                    continue;
                }

                if let Err(e) = input::handle_input(&mut app, key).await {
                    eprintln!("Error handling input: {}", e);
                }
            }
        }

        // Clear messages after they've been displayed for 3 seconds
        if app.error_message.is_some() || app.info_message.is_some() {
            if app.last_connection_attempt.elapsed() >= Duration::from_secs(3) {
                app.error_message = None;
                app.info_message = None;
            }
        }

        // Periodically retry connection if we're disconnected
        if app.connection_error.is_some() && app.last_connection_attempt.elapsed() >= RETRY_INTERVAL {
            if let Err(e) = app.attempt_connection().await {
                app.error_message = Some(format!("Connection attempt failed: {}", e));
                app.info_message = None;
            } else {
                app.info_message = Some("Connection attempt successful".to_string());
                app.error_message = None;
        }
    }
    }

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
