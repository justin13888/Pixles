package com.justin13888.pixles.models

enum class AlbumAccess {
    Owner,
    Write,
    Read,
}

/** Returns whether the access level grants write permission. */
fun AlbumAccess.isWrite(): Boolean = this == AlbumAccess.Owner || this == AlbumAccess.Write
