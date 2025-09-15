use std::path::PathBuf;
use std::time::Instant;

use capitalize::Capitalize;
use clap::Parser;
use cli::{AuthCommands, Cli, Commands};
use colored::*;
use dialoguer::Confirm;
use eyre::{Result, eyre};
use futures::stream::{self, StreamExt};
use pixles_core::metadata::FileMetadata;
use pixles_core::utils::hash::get_file_hash;
use tracing::{debug, trace};
use tracing_subscriber::prelude::*;
use tracing_subscriber::{EnvFilter, fmt};
use walkdir::WalkDir;

use crate::db::init_sqlite;
use crate::utils::directories::{get_cache_dir, get_config_dir, get_data_dir};

mod cli;
mod config;
mod db;
mod import;
mod status;
mod utils;

// #[tokio::main(flavor = "multi_thread")] # TODO: see what default for tokio
// runtime is ideal
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // Initialize tracing subscriber for logging
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| {
            if cfg!(debug_assertions) {
                EnvFilter::try_new("debug")
            } else {
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
                match config::Config::from_default_path() {
                    Ok(config) => match status::AuthStatus::check(&config).await {
                        Ok(auth_status) => {
                            auth_status.display();
                        }
                        Err(e) => {
                            println!("{}", format!("Error checking auth status: {e}").red());
                        }
                    },
                    Err(e) => {
                        println!("{}", format!("Error loading config: {e}").red());
                    }
                }
            }
        },
        Commands::Import { path } => {
            println!(
                "{}",
                format!("Importing from path: {}", path.to_string_lossy().blue()).green()
            );
            // TODO: Implement import logic

            // File or directory?
            let paths: Vec<PathBuf> = if path.is_dir() {
                println!("{}", "Importing from directory...".cyan());
                // Handle directory import

                WalkDir::new(&path)
                    .into_iter()
                    .filter_map(|entry| entry.ok())
                    .filter(|entry| entry.file_type().is_file())
                    .map(|entry| entry.path().to_path_buf())
                    .collect()
            } else if path.is_file() {
                println!("{}", "Importing from file...".cyan());
                // Handle file import
                vec![path]
            } else {
                println!("{}", "Invalid path provided".red());
                return Err(eyre!("Invalid path provided"));
            };

            println!(
                "{}",
                format!("Found {} files to import", paths.len()).green()
            );

            // Init local DB connection
            let db = init_sqlite().await?;
            debug!("Initialized SQLite database connection");

            // TODO: Detect file structures and summarize into ImportPlan
            // TODO: Show TUI for edit and confirm

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
            match status::StatusInfo::collect().await {
                Ok(status_info) => {
                    status_info.display();
                }
                Err(e) => {
                    println!("{}", format!("Error collecting status: {e}").red());
                }
            }
        }
        Commands::List { local, remote } => {
            println!("{}", "Listing files and albums...".green());
            if local {
                println!("{}", "Showing local files only".cyan());
            }
            if remote {
                println!("{}", "Showing remote files only".cyan());
            }
            todo!("Implement list logic");
        }
        Commands::Match { path } => {
            println!(
                "{}",
                format!(
                    "Matching metadata for file: {}",
                    path.to_string_lossy().blue()
                )
                .green()
            );

            if !path.exists() {
                return Err(eyre!("File does not exist: {}", path.to_string_lossy()));
            }

            if !path.is_file() {
                return Err(eyre!("Path is not a file: {}", path.to_string_lossy()));
            }

            // Get file metadata
            match FileMetadata::from_file_path(&path).await {
                Ok(metadata) => {
                    println!("{}", "File metadata:".green());
                    println!("{metadata:#?}");
                }
                Err(e) => {
                    return Err(eyre!("Failed to get file metadata: {}", e));
                }
            }

            // todo!("Implement match logic");
        }
        Commands::Reset {
            config,
            data,
            cache,
            all,
        } => {
            if !config && !data && !cache && !all {
                return Err(eyre!(
                    "No directories specified for reset. Use --all or specify at least one of \
                     --config, --data, --cache."
                ));
            }

            println!("{}", "Resetting all local CLI data...".red());
            let config_dir = get_config_dir().ok_or(eyre!("Failed to get config directory"))?;
            let data_dir = get_data_dir().ok_or(eyre!("Failed to get data directory"))?;
            let cache_dir = get_cache_dir().ok_or(eyre!("Failed to get cache directory"))?;

            let mut paths_to_remove = Vec::new();
            if all {
                paths_to_remove.push(("config", config_dir));
                paths_to_remove.push(("data", data_dir));
                paths_to_remove.push(("cache", cache_dir));
            } else {
                if config {
                    paths_to_remove.push(("config", config_dir));
                }
                if data {
                    paths_to_remove.push(("data", data_dir));
                }
                if cache {
                    paths_to_remove.push(("cache", cache_dir));
                }
            }

            // Prompt user for confirmation for each path
            for (label, path) in paths_to_remove {
                if path.exists() {
                    // Assert path is a directory
                    assert!(
                        path.is_dir(),
                        "Path {} is not a directory",
                        path.to_string_lossy()
                    );

                    // Prompt user for confirmation
                    let prompt = format!(
                        "Are you sure you want to delete the {} directory?\n  Path: {}",
                        label,
                        path.to_string_lossy()
                    );

                    if Confirm::new()
                        .with_prompt(&prompt)
                        .default(false)
                        .interact()?
                    {
                        println!("{}", format!("Removing {label} directory...").yellow());
                        match std::fs::remove_dir_all(&path) {
                            Ok(_) => println!(
                                "{}",
                                format!("Successfully removed {label} directory").green()
                            ),
                            Err(e) => println!(
                                "{}",
                                format!("Failed to remove {label} directory: {e}").red()
                            ),
                        }
                    } else {
                        println!("{}", format!("Skipping {label} directory").cyan());
                    }
                } else {
                    println!(
                        "{}",
                        format!(
                            "{} directory {} does not exist, skipping...",
                            label.capitalize_first_only(),
                            path.to_string_lossy()
                        )
                        .yellow()
                    );
                    continue;
                }
            }
        }
    }

    Ok(())
}
