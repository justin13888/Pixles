package com.justin13888.pixles.metadata

enum class AssetType {
    /** Photos */
    Photo,
    /** Videos */
    Video,
    /** Sidecars (related media files, e.g. XMP, JSON) */
    Sidecar,
}

/**
 * Detects asset type based on file extension. Returns null if not recognized.
 * Does not check whether the path actually exists or is a file.
 *
 * @param path file path (used only for extension detection)
 */
fun AssetType.Companion.fromFilePath(path: String): AssetType? {
    val ext = path.substringAfterLast('.', "").lowercase()
    return when (ext) {
        "jpg", "jpeg", "png", "gif" -> AssetType.Photo
        "mp4", "mov", "avi" -> AssetType.Video
        else -> null
    }
}
