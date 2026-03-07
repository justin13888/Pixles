package com.justin13888.pixles.models

import kotlin.uuid.Uuid

/**
 * Represents a media asset in Pixles.
 *
 * @param id Asset ID
 * @param albumId Album ID this asset belongs to, if any
 * @param ownerId Owner ID
 * @param ext File extension (e.g., "png", "mp4", "json"). Do NOT prepend with a dot.
 *            String is case-sensitive.
 */
data class Asset(
    val id: Uuid,
    val albumId: String?,
    val ownerId: String,
    val ext: String,
) {
    companion object {
        fun new(albumId: String?, ownerId: String, ext: String): Asset =
            Asset(id = Uuid.random(), albumId = albumId, ownerId = ownerId, ext = ext)
    }
}
