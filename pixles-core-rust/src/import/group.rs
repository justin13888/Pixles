// TODO: Group by
// - Similar photos
// - Burst photos
// - Stacked photos (e.g. from cameras)

use std::{collections::HashMap, path::PathBuf};

use log::{debug, trace};
use thiserror::Error;
use uuid::Uuid;

use crate::import::{ImportAction, ImportActionPlan};

pub type Grouping = Vec<Vec<PathBuf>>;
pub type GroupingResult<T> = Result<T, GroupingError>;

#[derive(Debug, Error)]
pub enum GroupingError {
    #[error("Failed to parse path ({0}): {1}")]
    PathParseError(PathBuf, String),
    #[error("Failed to find path: {0}")]
    PathNotFound(PathBuf),
    #[error("Unexpected inconsistency while grouping paths: {0}")]
    Inconsistency(String),
}

/// Returns list of paths that are grouped by their names.
/// E.g. `/path/to/a.jpg`, `/path/to/a.ARW` will be grouped together.
pub fn detect_groups_by_name(plan: &ImportActionPlan) -> Result<Grouping, GroupingError> {
    // parent -> [(full path, filestem), ...]
    let mut paths_by_parent: HashMap<Option<PathBuf>, Vec<(PathBuf, String)>> = HashMap::new();
    for path in plan.mapping().keys() {
        let parent = path.parent().map(|p| p.to_path_buf());
        let filestem = path
            .file_stem()
            .ok_or_else(|| {
                GroupingError::PathParseError(path.clone(), "Invalid file stem".to_string())
            })?
            .to_os_string()
            .to_string_lossy()
            .into_owned();
        paths_by_parent
            .entry(parent)
            .or_default()
            .push((path.clone(), filestem));
    }

    let mut groups: Grouping = Vec::new();

    // For each parent directory, group paths by their file names
    for (_parent, paths) in paths_by_parent.into_iter() {
        // filestem -> full path
        let filestems: HashMap<String, Vec<PathBuf>> =
            paths
                .into_iter()
                .fold(HashMap::new(), |mut acc, (full_path, filestem)| {
                    acc.entry(filestem).or_default().push(full_path);
                    acc
                });

        // Add grouped paths to the result
        filestems.into_iter().for_each(|(_filestem, full_paths)| {
            if full_paths.len() > 1 {
                // Only add groups with more than one path
                groups.push(full_paths);
            }
        });
    }

    Ok(groups)
}

pub const GROUP_FUNCS: &[fn(&ImportActionPlan) -> GroupingResult<Grouping>] =
    &[detect_groups_by_name];

/// Applies standard grouping rules to the import action plan.
/// This function will modify the plan in place, grouping paths based on the rules defined.
pub fn apply_grouping_rules(plan: &mut ImportActionPlan) -> GroupingResult<()> {
    trace!("Applying grouping rules to import action plan");

    for group_func in GROUP_FUNCS {
        let groups: Grouping = group_func(plan)?;
        if !groups.is_empty() {
            // Apply the grouping logic
            apply_grouped_paths(plan, groups)?;
        }
    }

    Ok(())
}

fn apply_grouped_paths(plan: &mut ImportActionPlan, groups: Grouping) -> Result<(), GroupingError> {
    for group in groups {
        if group.len() > 1 {
            // Verify they all exist in the plan
            for path in &group {
                if !plan.mapping().contains_key(path) {
                    return Err(GroupingError::PathNotFound(path.clone()));
                }
            }

            // Create a new group
            let group_id = Uuid::now_v7();

            // Update the plan to mark these paths as part of the group
            for path in group {
                if let Some((action, _scan_result)) = plan.mapping_mut().get_mut(&path) {
                    match action {
                        Some(ImportAction::New(config)) => {
                            if config.group_id.is_some() {
                                return Err(GroupingError::Inconsistency(
                                    "Path already has a group ID assigned".to_string(),
                                ));
                            }

                            // Assign the group ID to the new import config
                            config.group_id = Some(group_id.to_string());
                        }
                        _ => {
                            return Err(GroupingError::Inconsistency(format!(
                                "Path {} is not a new import action. Found {action:?}.",
                                path.display()
                            )));
                        }
                    }
                } else {
                    debug!("Path not found in plan: {}", path.display());
                    return Err(GroupingError::PathNotFound(path));
                }
            }
        }
    }
    Ok(())
}

// TODO: Write unit tests
