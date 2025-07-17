use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Authentication commands
    Auth {
        #[command(subcommand)]
        command: AuthCommands,
    },
    /// Import files to Pixles
    Import {
        /// Path to the file or directory to import
        path: String,
        /// Album name to import to
        #[arg(long)]
        album: Option<String>,
        /// Perform a dry run without making changes
        #[arg(long)]
        dry_run: bool,
    },
    /// Sync local and remote data
    Sync {
        /// Force sync even if there are conflicts
        #[arg(long)]
        force: bool,
        /// Perform a dry run without making changes
        #[arg(long)]
        dry_run: bool,
    },
    /// Show current status
    Status,
    /// List files and albums
    List {
        /// Show only local files
        #[arg(long)]
        local: bool,
        /// Show only remote files
        #[arg(long)]
        remote: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum AuthCommands {
    /// Login to Pixles
    Login,
    /// Logout from Pixles
    Logout,
    /// Show authentication status
    Status,
}
