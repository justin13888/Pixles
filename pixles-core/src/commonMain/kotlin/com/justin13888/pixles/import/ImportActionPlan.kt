package com.justin13888.pixles.import

import com.justin13888.pixles.import.group.detectGroupsByName

/** Maps file path → (selected action, scan result) */
typealias ImportActionMapping = LinkedHashMap<String, Pair<ImportAction?, ScanResult>>

/** Result type for grouping operations. */
typealias GroupingResult<T> = Result<T>

class ImportActionPlan(
    private val mapping: ImportActionMapping = LinkedHashMap(),
) {
    fun mapping(): ImportActionMapping = mapping

    fun len(): Int = mapping.size

    fun isEmpty(): Boolean = mapping.isEmpty()

    /**
     * Applies standard grouping rules to the import action plan, assigning group IDs
     * to files that share the same stem within the same directory.
     */
    fun applyGroupingRules(): GroupingResult<Unit> = runCatching {
        val groups = detectGroupsByName(this).getOrThrow()
        if (groups.isNotEmpty()) {
            applyGroupedPaths(groups).getOrThrow()
        }
    }

    private fun applyGroupedPaths(groups: List<List<String>>): GroupingResult<Unit> = runCatching {
        for (group in groups) {
            if (group.size > 1) {
                // Verify all paths exist in the plan
                for (path in group) {
                    check(mapping.containsKey(path)) { "Path not found in plan: $path" }
                }

                val groupId = kotlin.uuid.Uuid.random().toString()

                for (path in group) {
                    val (action, scanResult) = mapping[path]!!
                    when (action) {
                        is ImportAction.New -> {
                            check(action.config.groupId == null) {
                                "Path already has a group ID assigned: $path"
                            }
                            mapping[path] = Pair(
                                ImportAction.New(action.config.copy(groupId = groupId)),
                                scanResult,
                            )
                        }
                        else -> error("Path $path is not a new import action")
                    }
                }
            }
        }
    }
}
