package com.justin13888.pixles.import

import com.justin13888.pixles.metadata.AssetType

sealed class ImportAction {
    /** Import the file into the collection as a new asset. */
    data class New(val config: NewImportConfig) : ImportAction()
    /** Skip the file, leaving it unchanged. */
    data object Skip : ImportAction()
}

data class NewImportConfig(
    /** Asset type */
    val assetType: AssetType,
    /** Album ID to import into */
    val albumId: String? = null,
    /** Group ID to import into */
    val groupId: String? = null,
) {
    companion object {
        fun new(assetType: AssetType): NewImportConfig = NewImportConfig(assetType = assetType)
    }
}
