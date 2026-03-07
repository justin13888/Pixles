package com.justin13888.pixles.metadata

import kotlinx.datetime.Instant

data class FileMetadata(
    /** XXH64 hash of the file contents */
    val hashXxh64: ULong,
    /** File size in bytes */
    val size: ULong,
    /** Original file name */
    val originalFilename: String,
    /** File creation timestamp */
    val createdAt: Instant,
    /** Last modified timestamp */
    val modifiedAt: Instant,
    /** Timestamp when the file was imported */
    val importedAt: Instant,
)
