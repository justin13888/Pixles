package com.justin13888.pixles.import

sealed class SpecialStatus {
    data class File(val status: SpecialFileStatus) : SpecialStatus()
    data class Directory(val status: SpecialDirectoryStatus) : SpecialStatus()
}

enum class SpecialFileStatus {
    Dxo;

    companion object {
        /**
         * Detects from a file name/path whether it is a special file.
         * Does not verify the path is actually a file on disk.
         */
        fun fromPath(path: String): SpecialFileStatus? {
            val filename = path.substringAfterLast('/')
            return when (filename) {
                "dxo" -> Dxo
                else -> null
            }
        }
    }
}

enum class SpecialDirectoryStatus {
    DavinciResolve,
    Git;

    companion object {
        /**
         * Detects from a directory name/path whether it is a special directory.
         * Does not verify the path is actually a directory on disk.
         */
        fun fromPath(path: String): SpecialDirectoryStatus? {
            val filename = path.substringAfterLast('/')
            return when (filename) {
                ".git" -> Git
                ".dra" -> DavinciResolve
                else -> null
            }
        }
    }
}
