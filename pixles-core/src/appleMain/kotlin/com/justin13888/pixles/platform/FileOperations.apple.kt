package com.justin13888.pixles.platform

import com.justin13888.pixles.metadata.FileMetadata
import com.justin13888.pixles.utils.XxHash64
import kotlinx.cinterop.ExperimentalForeignApi
import kotlinx.cinterop.addressOf
import kotlinx.cinterop.usePinned
import kotlinx.datetime.Clock
import kotlinx.datetime.Instant
import platform.Foundation.NSData
import platform.Foundation.NSDate
import platform.Foundation.NSFileCreationDate
import platform.Foundation.NSFileManager
import platform.Foundation.NSFileModificationDate
import platform.Foundation.NSFileSize
import platform.Foundation.dataWithContentsOfFile
import platform.Foundation.lastPathComponent
import platform.Foundation.timeIntervalSince1970

@OptIn(ExperimentalForeignApi::class)
actual fun getFileHash(path: String): ULong {
    val data = NSData.dataWithContentsOfFile(path)
        ?: throw IllegalStateException("Cannot read file: $path")
    val bytes = ByteArray(data.length.toInt())
    bytes.usePinned { pinned ->
        platform.posix.memcpy(pinned.addressOf(0), data.bytes, data.length)
    }
    return XxHash64.hash(bytes)
}

@OptIn(ExperimentalForeignApi::class)
actual fun getFileMetadata(path: String): FileMetadata {
    val manager = NSFileManager.defaultManager()
    val attrs = manager.attributesOfItemAtPath(path, error = null)
        ?: throw IllegalStateException("Cannot read metadata for: $path")

    val data = NSData.dataWithContentsOfFile(path)
        ?: throw IllegalStateException("Cannot read file: $path")
    val bytes = ByteArray(data.length.toInt())
    bytes.usePinned { pinned ->
        platform.posix.memcpy(pinned.addressOf(0), data.bytes, data.length)
    }

    val hash = XxHash64.hash(bytes)
    val size = (attrs[NSFileSize] as? Number)?.toLong()?.toULong() ?: 0UL

    val createdAt = (attrs[NSFileCreationDate] as? NSDate)
        ?.timeIntervalSince1970
        ?.let { Instant.fromEpochMilliseconds((it * 1000).toLong()) }
        ?: Clock.System.now()

    val modifiedAt = (attrs[NSFileModificationDate] as? NSDate)
        ?.timeIntervalSince1970
        ?.let { Instant.fromEpochMilliseconds((it * 1000).toLong()) }
        ?: Clock.System.now()

    return FileMetadata(
        hashXxh64 = hash,
        size = size,
        originalFilename = path.lastPathComponent,
        createdAt = createdAt,
        modifiedAt = modifiedAt,
        importedAt = Clock.System.now(),
    )
}
