pub mod input;
pub mod snapcast;
pub mod ui;

use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::{
    env,
    io,
    process,
};
use clap::{Arg, Command};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "1780";

#[derive(Debug)]
pub struct App {
    pub should_quit: bool,
    pub snapcast_client: snapcast::SnapcastClient,
    pub selected_item: Option<usize>,
    pub current_tab: usize,
}

fn get_snapserver_host() -> String {
    env::var("SNAPSERVER_HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string())
}

fn get_snapserver_port() -> String {
    env::var("SNAPSERVER_PORT").unwrap_or_else(|_| DEFAULT_PORT.to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // First check if the user just wants the version
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && args[1] == "version" {
        println!("Version: v{}", VERSION);
        process::exit(0);
    }

    // Parse command line arguments
    let matches = Command::new("snap-tui")
        .version(VERSION)
        .author("Your Name <your.email@example.com>")
        .about("A TUI for Snapcast")
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

    // Handle version flag (for --version or -V)
    if matches.contains_id("version") {
        println!("Version: v{}", VERSION);
        process::exit(0);
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
    };

    // Initial data fetch
    if let Err(e) = app.snapcast_client.fetch_status().await {
        eprintln!("Error fetching initial status: {}", e);
        process::exit(1);
    }

    // Main loop
    while !app.should_quit {
        terminal.draw(|f| ui::ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if let Err(e) = input::handle_input(&mut app, key).await {
                eprintln!("Error handling input: {}", e);
            }
        }
    }

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}