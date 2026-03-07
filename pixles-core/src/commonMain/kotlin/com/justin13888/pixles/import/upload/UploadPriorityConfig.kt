package com.justin13888.pixles.import.upload

// Related documentation: https://pixles.justinchung.net/design/import-prioritization/

data class UploadPriorityConfig(
    /** Whether to prioritize smaller files first */
    val prioritizeSmallerFiles: Boolean = true,
    /** Whether to prioritize newer files first */
    val prioritizeNewerFiles: Boolean = true,
    /** Whether to prioritize files with lower directory depth first */
    val prioritizeLowerDepth: Boolean = true,
)

// TODO: Implement get_upload_ordering equivalent once full metadata is available in the plan
