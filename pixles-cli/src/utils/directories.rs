use std::path::PathBuf;

use directories::ProjectDirs;

/// Returns project directory for the Pixles CLI.
fn get_project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "Pixles", "Pixles CLI")
}

/// Returns the default configuration directory for the Pixles CLI.
pub fn get_config_dir() -> Option<PathBuf> {
    get_project_dirs().map(|proj_dirs| proj_dirs.config_dir().to_path_buf())
}

/// Returns the config file path for the Pixles CLI.
pub fn get_config_file_path() -> Option<PathBuf> {
    get_project_dirs().map(|proj_dirs| proj_dirs.preference_dir().join("config.toml"))
}

/// Returns the default cache directory for the Pixles CLI.
pub fn get_cache_dir() -> Option<PathBuf> {
    get_project_dirs().map(|proj_dirs| proj_dirs.cache_dir().to_path_buf())
}

/// Returns the default data directory for the Pixles CLI.
pub fn get_data_dir() -> Option<PathBuf> {
    get_project_dirs().map(|proj_dirs| proj_dirs.data_dir().to_path_buf())
}

/// Returns the sync directory for the Pixles CLI.
pub fn get_sync_dir() -> Option<PathBuf> {
    get_data_dir().map(|dir| dir.join("sync"))
}
