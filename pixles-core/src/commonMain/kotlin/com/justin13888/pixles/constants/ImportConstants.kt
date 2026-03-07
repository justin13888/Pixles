package com.justin13888.pixles.constants

val IGNORE_RULES: List<String> = listOf(
    // Ignore hidden files and directories
    ".*",
    // Ignore system files
    "*.DS_Store",
)

val SIDECAR_EXTENSIONS: List<String> = listOf(
    "xmp",  // Custom formats
    "json", // Commonly used to append metadata to media files
    "xml",
)
