use colored::*;
use eyre::Result;
use tracing::trace;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use clap::Parser;
use cli::{AuthCommands, Cli, Commands};

mod cli;

// #[tokio::main(flavor = "multi_thread")] # TODO: see what default for tokio runtime is ideal
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber for logging
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();
    let fmt_layer = fmt::layer()
        .pretty()
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true);
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    // Start parsing
    let cli = Cli::parse();

    trace!("Parsed CLI arguments: {:#?}", cli);

    match cli.command {
        Commands::Auth { command } => match command {
            AuthCommands::Login => {
                println!("{}", "Logging in to Pixles...".green());
                todo!("Implement login logic");
            }
            AuthCommands::Logout => {
                println!("{}", "Logging out from Pixles...".yellow());
                todo!("Implement logout logic");
            }
            AuthCommands::Status => {
                println!("{}", "Checking authentication status...".blue());
                todo!("Implement status logic");
            }
        },
        Commands::Import {
            path,
            album,
            dry_run,
        } => {
            println!("{}", format!("Importing from path: {path}").green());
            if let Some(album_name) = album {
                println!("{}", format!("Target album: {album_name}").cyan());
            }
            if dry_run {
                println!("{}", "Dry run mode enabled".yellow());
            }
            // TODO: Implement import logic
        }
        Commands::Sync { force, dry_run } => {
            println!("{}", "Syncing local and remote data...".green());
            if force {
                println!("{}", "Force sync enabled".yellow());
            }
            if dry_run {
                println!("{}", "Dry run mode enabled".yellow());
            }
            // TODO: Implement sync logic
        }
        Commands::Status => {
            println!("{}", "Checking Pixles status...".blue());
            // TODO: Implement status logic
        }
        Commands::List { local, remote } => {
            println!("{}", "Listing files and albums...".green());
            if local {
                println!("{}", "Showing local files only".cyan());
            }
            if remote {
                println!("{}", "Showing remote files only".cyan());
            }
            // TODO: Implement list logic
        }
    }

    Ok(())
}
