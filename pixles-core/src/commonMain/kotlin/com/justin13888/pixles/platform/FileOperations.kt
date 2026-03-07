package com.justin13888.pixles.platform

import com.justin13888.pixles.metadata.FileMetadata

/**
 * Reads the file at [path] and returns its XXH64 hash.
 * @throws Exception if the file cannot be read.
 */
expect fun getFileHash(path: String): ULong

/**
 * Reads the file system metadata for the file at [path].
 * @throws Exception if the file cannot be read or does not exist.
 */
expect fun getFileMetadata(path: String): FileMetadata
