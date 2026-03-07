package com.justin13888.pixles.import

sealed class ImportResult {
    /** The file was successfully imported. */
    data object Success : ImportResult()
    /** The file was skipped. */
    data object Skipped : ImportResult()
    /** An error occurred during import. */
    data class Error(val message: String) : ImportResult()
}
