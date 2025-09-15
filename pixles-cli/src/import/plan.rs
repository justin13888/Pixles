use std::path::PathBuf;

use eyre::{Result, eyre};
use indexmap::IndexMap;
use pixles_core::import::{
    ImportAction, ImportActionMapping, ImportActionPlan, ImportExecutionPlan,
    ImportExecutionSummary, ImportResult, NewImportConfig, ScanResult, SpecialDirectoryStatus,
    SpecialFileStatus,
};
use pixles_core::metadata::AssetType;

/// Scan files and create an import plan.
pub fn create_import_plan(file: PathBuf) -> Result<ImportActionPlan> {
    // Parse path based on file or directory
    let file_paths: Vec<PathBuf> = if file.is_dir() {
        // Scan directory and ensure there aren't subdirectories
        let entries = std::fs::read_dir(&file)
            .map_err(|_| eyre!("Failed to read directory: {}", file.display()))?;

        // If there is a subdirectory that isn't a special directory, set it as
        // unknown
        let mut paths = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|_| eyre!("Failed to read directory entry"))?;
            let path = entry.path();
            paths.push(path);
        }

        paths
    } else {
        // Single file
        vec![file]
    };

    let mut mapping: ImportActionMapping = IndexMap::new();
    for path in file_paths.into_iter() {
        // TODO: Incorporate metadata to detect whether item is to skip
        let scan_result: ScanResult = if path.is_dir() {
            let is_special = SpecialDirectoryStatus::from_path(&path);
            ScanResult::Directory {
                detected_asset_type: if is_special.is_some() {
                    Some(AssetType::Sidecar)
                } else {
                    None
                },
                is_special,
            }
        } else {
            let is_special = SpecialFileStatus::from_path(&path);
            let detected_asset_type = AssetType::from_file_path(&path);

            ScanResult::File {
                detected_asset_type,
                is_special,
            }
        };

        // TODO: handle case when scanresult indicates file/dir is to be ignored vv
        let import_action: Option<ImportAction> = match &scan_result {
            ScanResult::File {
                detected_asset_type,
                is_special,
            } => match detected_asset_type {
                None => {
                    if is_special.is_some() {
                        Some(ImportAction::New(NewImportConfig::new(AssetType::Sidecar)))
                    } else {
                        None
                    }
                }
                Some(detected_asset_type) => Some(ImportAction::New(NewImportConfig::new(
                    *detected_asset_type,
                ))),
            },
            ScanResult::Directory {
                detected_asset_type,
                is_special,
            } => match detected_asset_type {
                None => {
                    if is_special.is_some() {
                        Some(ImportAction::New(NewImportConfig::new(AssetType::Sidecar)))
                    } else {
                        None
                    }
                }
                Some(detected_asset_type) => Some(ImportAction::New(NewImportConfig::new(
                    *detected_asset_type,
                ))),
            },
        };

        mapping.insert(path, (import_action, scan_result));
    }

    let plan = ImportActionPlan::new(mapping);

    Ok(plan)
}

/// Executes plan and returns the result of the import.
pub fn execute_import_plan(plan: &ImportExecutionPlan) -> ImportExecutionSummary {
    // TODO: Actually implement the import logic

    // Assumptions: Import execution plan is initialized properly, mapping is correct and normalized.
    // Strategy:
    // - Start importing queue, with

    let summary: Vec<(PathBuf, ImportResult)> = plan
        .mapping()
        .iter()
        .map(|(file, action)| {
            match action {
                ImportAction::New(config) => {
                    // Here you would implement the logic to import the file
                    // For now, we just return a success result
                    (file.clone(), ImportResult::Success)
                }
                ImportAction::Skip => {
                    // Skip the file
                    (file.clone(), ImportResult::Skipped)
                }
            }
        })
        .collect();

    assert_eq!(
        summary.len(),
        plan.mapping().len(),
        "Summary length must match mapping length"
    );

    ImportExecutionSummary(summary)
}

// TODO: Could make this more elegant ^^
