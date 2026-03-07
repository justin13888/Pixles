package com.justin13888.pixles.utils

/**
 * Checks whether any path in the list is an ancestor of another path in the list.
 * Assumes all paths are absolute and normalized (no trailing slashes, no `..` components).
 * Returns true if nested paths are detected (which is an error condition for import plans).
 */
fun areThereNestedPaths(paths: List<String>): Boolean {
    if (paths.size <= 1) return false

    // Normalize: ensure no trailing slash
    val normalized = paths.map { it.trimEnd('/') }.sorted()

    for (i in 0 until normalized.size - 1) {
        val current = normalized[i]
        val next = normalized[i + 1]
        // `next` is nested under `current` if it starts with current + "/"
        if (next.startsWith("$current/")) return true
    }

    return false
}
