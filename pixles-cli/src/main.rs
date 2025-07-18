use clap::Parser;
use cli::{AuthCommands, Cli, Commands};
use colored::*;
use eyre::Result;
use tracing::trace;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{EnvFilter, fmt};

mod cli;
mod config;
mod status;
mod utils;

// #[tokio::main(flavor = "multi_thread")] # TODO: see what default for tokio
// runtime is ideal
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

    match cli.command
    {
        Commands::Auth { command } => match command
        {
            AuthCommands::Login =>
            {
                println!("{}", "Logging in to Pixles...".green());
                todo!("Implement login logic");
            }
            AuthCommands::Logout =>
            {
                println!("{}", "Logging out from Pixles...".yellow());
                todo!("Implement logout logic");
            }
            AuthCommands::Status =>
            {
                println!("{}", "Checking authentication status...".blue());
                match config::Config::from_default_path()
                {
                    Ok(config) => match status::AuthStatus::check(&config).await
                    {
                        Ok(auth_status) =>
                        {
                            auth_status.display();
                        }
                        Err(e) =>
                        {
                            println!("{}", format!("Error checking auth status: {e}").red());
                        }
                    },
                    Err(e) =>
                    {
                        println!("{}", format!("Error loading config: {e}").red());
                    }
                }
            }
        },
        Commands::Import {
            path,
            album,
            dry_run,
        } =>
        {
            println!("{}", format!("Importing from path: {path}").green());
            if let Some(album_name) = album
            {
                println!("{}", format!("Target album: {album_name}").cyan());
            }
            if dry_run
            {
                println!("{}", "Dry run mode enabled".yellow());
            }
            // TODO: Implement import logic
        }
        Commands::Sync { force, dry_run } =>
        {
            println!("{}", "Syncing local and remote data...".green());
            if force
            {
                println!("{}", "Force sync enabled".yellow());
            }
            if dry_run
            {
                println!("{}", "Dry run mode enabled".yellow());
            }
            // TODO: Implement sync logic
        }
        Commands::Status =>
        {
            println!("{}", "Checking Pixles status...".blue());
            match status::StatusInfo::collect().await
            {
                Ok(status_info) =>
                {
                    status_info.display();
                }
                Err(e) =>
                {
                    println!("{}", format!("Error collecting status: {}", e).red());
                }
            }
        }
        Commands::List { local, remote } =>
        {
            println!("{}", "Listing files and albums...".green());
            if local
            {
                println!("{}", "Showing local files only".cyan());
            }
            if remote
            {
                println!("{}", "Showing remote files only".cyan());
            }
            // TODO: Implement list logic
        }
    }

    Ok(())
}
