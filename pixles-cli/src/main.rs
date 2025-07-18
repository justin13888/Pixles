use std::path::PathBuf;
use std::time::Instant;

use clap::Parser;
use cli::{AuthCommands, Cli, Commands};
use colored::*;
use eyre::{Result, eyre};
use futures::stream::{self, StreamExt};
use sea_orm::Database;
use tracing::{debug, trace};
use tracing_subscriber::prelude::*;
use tracing_subscriber::{EnvFilter, fmt};
use walkdir::WalkDir;

use crate::utils::directories::get_sqlite_db_path;
use crate::utils::hash::get_file_hash;

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
        .or_else(|_| {
            if cfg!(debug_assertions)
            {
                EnvFilter::try_new("debug")
            }
            else
            {
                EnvFilter::try_new("info")
            }
        })
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
            println!(
                "{}",
                format!("Importing from path: {}", path.to_string_lossy().blue()).green()
            );
            if let Some(album) = album
            {
                println!("{}", format!("Target album: {}", album.blue()).cyan());

                // TODO: Verify album exists by ID or name
            }
            if dry_run
            {
                println!("{}", "Dry run mode enabled".yellow());
                // TODO: Do something about dry run
            }
            // TODO: Implement import logic

            // File or directory?
            let paths: Vec<PathBuf> = if path.is_dir()
            {
                println!("{}", "Importing from directory...".cyan());
                // Handle directory import

                WalkDir::new(&path)
                    .into_iter()
                    .filter_map(|entry| entry.ok())
                    .filter(|entry| entry.file_type().is_file())
                    .map(|entry| entry.path().to_path_buf())
                    .collect()
            }
            else if path.is_file()
            {
                println!("{}", "Importing from file...".cyan());
                // Handle file import
                vec![path]
            }
            else
            {
                println!("{}", "Invalid path provided".red());
                return Err(eyre!("Invalid path provided"));
            };

            println!(
                "{}",
                format!("Found {} files to import", paths.len()).green()
            );

            // Init local DB connection
            let db_path = get_sqlite_db_path().ok_or(eyre!("Failed to get SQLite DB path"))?;
            {
                // Create the database directory if it doesn't exist
                if let Some(parent) = db_path.parent()
                {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| eyre!("Failed to create DB directory: {:?}", e))?;
                }
            }
            let db_url = format!("sqlite://{}?mode=rwc", db_path.to_string_lossy());
            debug!("{}", format!("Using SQLite DB at: {db_url}").blue());
            let db = Database::connect(&db_url).await?;
            // db.close().await?;
            debug!("Connected to SQLite database at: {db_url}");
            // TODO: Do migrations and stuff

            // Compute hashes for files
            let hash_start = Instant::now();
            let hashes = stream::iter(paths)
                .map(|p| {
                    let path = p.clone();
                    async move {
                        match get_file_hash(&path) {
                            Ok(hash) => Some((path, hash)),
                            Err(_) => None,
                        }
                    }
                })
                .buffer_unordered(10) // Process up to 10 files concurrently
                .filter_map(|result| async move { result })
                .collect::<Vec<(PathBuf, u64)>>()
                .await;
            let hash_duration = hash_start.elapsed();

            println!(
                "{}",
                format!(
                    "Computed hashes for {} files in {:2} s",
                    hashes.len(),
                    hash_duration.as_secs_f32()
                )
                .green()
            );

            // TODO: Finish this
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
                    println!("{}", format!("Error collecting status: {e}").red());
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
            todo!("Implement list logic");
        }
    }

    Ok(())
}
