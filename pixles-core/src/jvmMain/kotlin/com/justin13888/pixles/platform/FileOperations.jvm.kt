package com.justin13888.pixles.platform

import com.justin13888.pixles.metadata.FileMetadata
import com.justin13888.pixles.utils.XxHash64
import kotlinx.datetime.Instant
import java.io.File
import java.nio.file.Files
import java.nio.file.attribute.BasicFileAttributes

actual fun getFileHash(path: String): ULong {
    val bytes = File(path).readBytes()
    return XxHash64.hash(bytes)
}

actual fun getFileMetadata(path: String): FileMetadata {
    val file = File(path)
    val attrs = Files.readAttributes(file.toPath(), BasicFileAttributes::class.java)

    val bytes = file.readBytes()
    val hash = XxHash64.hash(bytes)

    return FileMetadata(
        hashXxh64 = hash,
        size = attrs.size().toULong(),
        originalFilename = file.name,
        createdAt = Instant.fromEpochMilliseconds(attrs.creationTime().toMillis()),
        modifiedAt = Instant.fromEpochMilliseconds(attrs.lastModifiedTime().toMillis()),
        importedAt = kotlinx.datetime.Clock.System.now(),
    )
}
