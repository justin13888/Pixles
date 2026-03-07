package com.justin13888.pixles.import

data class ImportExecutionSummary(
    val results: List<Pair<String, ImportResult>>,
) {
    /** Returns number of successful imports. */
    val successCount: Int get() = results.count { (_, result) -> result is ImportResult.Success }

    /** Returns total number of imports. */
    val totalCount: Int get() = results.size
}
