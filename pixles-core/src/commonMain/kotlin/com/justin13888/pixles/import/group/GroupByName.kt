package com.justin13888.pixles.import.group

import com.justin13888.pixles.import.GroupingResult
import com.justin13888.pixles.import.ImportActionPlan

typealias Grouping = List<List<String>>

/**
 * Groups paths by their file stem within the same parent directory.
 * E.g. `/path/to/a.jpg` and `/path/to/a.ARW` will be grouped together.
 */
fun detectGroupsByName(plan: ImportActionPlan): GroupingResult<Grouping> = runCatching {
    // parent → [(fullPath, fileStem)]
    val pathsByParent = mutableMapOf<String?, MutableList<Pair<String, String>>>()

    for (path in plan.mapping().keys) {
        val parent = path.substringBeforeLast('/', "").ifEmpty { null }
        val filename = path.substringAfterLast('/')
        val stem = filename.substringBeforeLast('.', filename)
        pathsByParent.getOrPut(parent) { mutableListOf() }.add(Pair(path, stem))
    }

    val groups = mutableListOf<List<String>>()

    for ((_, paths) in pathsByParent) {
        // stem → [fullPaths]
        val byStem = mutableMapOf<String, MutableList<String>>()
        for ((fullPath, stem) in paths) {
            byStem.getOrPut(stem) { mutableListOf() }.add(fullPath)
        }

        for ((_, group) in byStem) {
            if (group.size > 1) {
                groups.add(group)
            }
        }
    }

    groups
}
