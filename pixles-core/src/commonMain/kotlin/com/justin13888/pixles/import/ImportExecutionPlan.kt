package com.justin13888.pixles.import

import com.justin13888.pixles.utils.areThereNestedPaths

typealias ImportExecutionMapping = List<Pair<String, ImportAction>>

sealed class ImportExecutionPlanError(message: String) : Exception(message) {
    class EmptyPlan : ImportExecutionPlanError("Import action plan is empty")
    class NestedPaths : ImportExecutionPlanError("There are nested paths in the import plan")
    class NoActionForFile(path: String) : ImportExecutionPlanError("No action specified for file: $path")
}

class ImportExecutionPlan private constructor(
    private val mapping: MutableList<Pair<String, ImportAction>>,
) {
    fun mapping(): ImportExecutionMapping = mapping

    /** Returns a sequence of paths whose action is [ImportAction.New]. */
    fun getUploadablePaths(): Sequence<String> =
        mapping.asSequence().filter { (_, action) -> action is ImportAction.New }.map { it.first }

    /** Sorts the mapping by file path. */
    fun normalize(): ImportExecutionPlan {
        mapping.sortBy { it.first }
        return this
    }

    companion object {
        /**
         * Validates and converts an [ImportActionPlan] into an [ImportExecutionPlan].
         * Returns a failure if the plan is empty, contains nested paths, or has entries with no action.
         */
        fun from(plan: ImportActionPlan): Result<ImportExecutionPlan> = runCatching {
            if (plan.isEmpty()) throw ImportExecutionPlanError.EmptyPlan

            val paths = plan.mapping().keys.toList()
            if (areThereNestedPaths(paths)) throw ImportExecutionPlanError.NestedPaths

            val executionMapping = mutableListOf<Pair<String, ImportAction>>()
            for ((path, entry) in plan.mapping()) {
                val (action, _) = entry
                if (action == null) throw ImportExecutionPlanError.NoActionForFile(path)
                executionMapping.add(Pair(path, action))
            }

            ImportExecutionPlan(executionMapping).normalize()
        }
    }
}
