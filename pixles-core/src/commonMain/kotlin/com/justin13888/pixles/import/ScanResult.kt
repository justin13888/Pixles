package com.justin13888.pixles.import

import com.justin13888.pixles.metadata.AssetType

sealed class ScanResult {
    data class File(
        val detectedAssetType: AssetType?,
        val isSpecial: SpecialFileStatus?,
    ) : ScanResult()

    data class Directory(
        val detectedAssetType: AssetType?,
        val isSpecial: SpecialDirectoryStatus?,
    ) : ScanResult()
}
